//! Regression tests for Oracle row limiting (#480).
//!
//! Oracle has no `LIMIT`; the generator must render `Limit` nodes as
//! `[OFFSET n ROWS] FETCH FIRST m ROWS ONLY` wherever they appear, whether the
//! AST was built programmatically or parsed from another dialect.

use polyglot_sql::builder;
use polyglot_sql::expressions::Expression;
use polyglot_sql::generator::{GeneratorConfig, LimitFetchStyle};
use polyglot_sql::{parse, transpile, Dialect, DialectType, Generator};

fn oracle_config() -> GeneratorConfig {
    Dialect::get(DialectType::Oracle).generator_config().clone()
}

fn generate_oracle(ast: &Expression) -> String {
    Generator::with_config(oracle_config())
        .generate(ast)
        .expect("generate")
}

fn to_oracle(sql: &str, source: DialectType) -> String {
    transpile(sql, source, DialectType::Oracle)
        .unwrap_or_else(|e| panic!("transpile failed for {sql:?}: {e}"))
        .into_iter()
        .next()
        .expect("expected at least one statement")
}

#[test]
fn oracle_defaults_to_fetch_first_style() {
    assert_eq!(
        oracle_config().limit_fetch_style,
        LimitFetchStyle::FetchFirst
    );
}

#[test]
fn builder_limit_renders_fetch_first() {
    let ast = builder::from("t").select_cols(["a"]).limit(5).build();
    assert_eq!(
        generate_oracle(&ast),
        "SELECT a FROM t FETCH FIRST 5 ROWS ONLY"
    );
}

#[test]
fn builder_limit_offset_renders_offset_rows_fetch_first() {
    let ast = builder::from("t")
        .select_cols(["a"])
        .limit(5)
        .offset(10)
        .build();
    assert_eq!(
        generate_oracle(&ast),
        "SELECT a FROM t OFFSET 10 ROWS FETCH FIRST 5 ROWS ONLY"
    );
}

#[test]
fn parsed_limit_generated_directly_for_oracle() {
    let ast = parse("SELECT a FROM t LIMIT 5", DialectType::Generic)
        .expect("parse")
        .remove(0);
    assert_eq!(
        generate_oracle(&ast),
        "SELECT a FROM t FETCH FIRST 5 ROWS ONLY"
    );
}

#[test]
fn transpile_limit_to_oracle() {
    let cases = [
        ("SELECT a FROM t LIMIT 5", "SELECT a FROM t FETCH FIRST 5 ROWS ONLY"),
        (
            "SELECT a FROM t ORDER BY a LIMIT 5 OFFSET 10",
            "SELECT a FROM t ORDER BY a OFFSET 10 ROWS FETCH FIRST 5 ROWS ONLY",
        ),
        ("SELECT a FROM t OFFSET 10", "SELECT a FROM t OFFSET 10 ROWS"),
        // LIMIT ALL / LIMIT NULL mean "no limit".
        ("SELECT a FROM t LIMIT ALL", "SELECT a FROM t"),
        ("SELECT a FROM t LIMIT NULL", "SELECT a FROM t"),
        // Nested queries, not just the top-level SELECT.
        (
            "SELECT * FROM (SELECT a FROM t LIMIT 5) AS s",
            "SELECT * FROM (SELECT a FROM t FETCH FIRST 5 ROWS ONLY) s",
        ),
        (
            "SELECT a FROM t WHERE a IN (SELECT b FROM u LIMIT 3)",
            "SELECT a FROM t WHERE a IN (SELECT b FROM u FETCH FIRST 3 ROWS ONLY)",
        ),
        (
            "WITH c AS (SELECT a FROM t LIMIT 2) SELECT * FROM c",
            "WITH c AS (SELECT a FROM t FETCH FIRST 2 ROWS ONLY) SELECT * FROM c",
        ),
        (
            "INSERT INTO x SELECT a FROM t LIMIT 5",
            "INSERT INTO x SELECT a FROM t FETCH FIRST 5 ROWS ONLY",
        ),
        // Set operations.
        (
            "SELECT a FROM t UNION ALL SELECT b FROM u LIMIT 5",
            "SELECT a FROM t UNION ALL SELECT b FROM u FETCH FIRST 5 ROWS ONLY",
        ),
        (
            "SELECT a FROM t UNION ALL SELECT b FROM u ORDER BY 1 LIMIT 5 OFFSET 2",
            "SELECT a FROM t UNION ALL SELECT b FROM u ORDER BY 1 OFFSET 2 ROWS FETCH FIRST 5 ROWS ONLY",
        ),
        (
            "SELECT a FROM t INTERSECT SELECT b FROM u LIMIT 5",
            "SELECT a FROM t INTERSECT SELECT b FROM u FETCH FIRST 5 ROWS ONLY",
        ),
        (
            "SELECT a FROM t EXCEPT SELECT b FROM u LIMIT 5",
            "SELECT a FROM t MINUS SELECT b FROM u FETCH FIRST 5 ROWS ONLY",
        ),
    ];
    for (sql, expected) in cases {
        assert_eq!(to_oracle(sql, DialectType::PostgreSQL), expected, "{sql}");
    }
}

#[test]
fn transpile_tsql_top_to_oracle() {
    let cases = [
        (
            "SELECT TOP 5 a FROM t",
            "SELECT a FROM t FETCH FIRST 5 ROWS ONLY",
        ),
        (
            "SELECT TOP 10 PERCENT a FROM t",
            "SELECT a FROM t FETCH FIRST 10 PERCENT ROWS ONLY",
        ),
        (
            "SELECT TOP 5 WITH TIES a FROM t ORDER BY a",
            "SELECT a FROM t ORDER BY a NULLS FIRST FETCH FIRST 5 ROWS WITH TIES",
        ),
    ];
    for (sql, expected) in cases {
        assert_eq!(to_oracle(sql, DialectType::TSQL), expected, "{sql}");
    }
}

#[test]
fn oracle_fetch_identity_is_preserved() {
    for sql in [
        "SELECT a FROM t FETCH FIRST 5 ROWS ONLY",
        "SELECT a FROM t OFFSET 1 ROWS FETCH NEXT 5 ROWS ONLY",
        "SELECT a FROM t ORDER BY a FETCH FIRST 5 ROWS WITH TIES",
    ] {
        assert_eq!(to_oracle(sql, DialectType::Oracle), sql);
    }
}

#[test]
fn pretty_oracle_limit_offset() {
    let ast = parse(
        "SELECT a FROM t ORDER BY a LIMIT 5 OFFSET 10",
        DialectType::PostgreSQL,
    )
    .expect("parse")
    .remove(0);
    let mut config = oracle_config();
    config.pretty = true;
    let sql = Generator::with_config(config)
        .generate(&ast)
        .expect("generate");
    assert_eq!(
        sql,
        "SELECT\n  a\nFROM t\nORDER BY\n  a\nOFFSET 10 ROWS\nFETCH FIRST 5 ROWS ONLY"
    );
}

#[test]
fn fetch_first_style_applies_to_any_generator_config() {
    let ast = builder::from("t").select_cols(["a"]).limit(5).build();
    let config = GeneratorConfig {
        limit_fetch_style: LimitFetchStyle::FetchFirst,
        ..Default::default()
    };
    let sql = Generator::with_config(config)
        .generate(&ast)
        .expect("generate");
    assert_eq!(sql, "SELECT a FROM t FETCH FIRST 5 ROWS ONLY");
}

#[test]
fn limit_style_dialects_still_emit_limit() {
    for dialect in [
        DialectType::PostgreSQL,
        DialectType::DuckDB,
        DialectType::MySQL,
    ] {
        let out = transpile(
            "SELECT a FROM t LIMIT 5 OFFSET 2",
            DialectType::Generic,
            dialect,
        )
        .expect("transpile")
        .remove(0);
        assert_eq!(out, "SELECT a FROM t LIMIT 5 OFFSET 2", "{dialect:?}");
    }
}
