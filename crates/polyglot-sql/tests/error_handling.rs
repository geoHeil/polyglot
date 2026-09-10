//! Error Handling Tests
//!
//! Tests for error scenarios to ensure graceful failure and useful error messages.

use polyglot_sql::dialects::{Dialect, DialectType};
use polyglot_sql::generator::Generator;
use polyglot_sql::parser::Parser;

// ============================================================================
// Syntax Error Tests
// ============================================================================

mod syntax_errors {
    use super::*;

    fn parse_with_dialect(
        sql: &str,
        dialect: DialectType,
    ) -> polyglot_sql::error::Result<Vec<polyglot_sql::Expression>> {
        Dialect::get(dialect).parse(sql)
    }

    #[test]
    fn test_unbalanced_parentheses_open() {
        let result = Parser::parse_sql("SELECT (1 + 2");
        assert!(result.is_err(), "Expected error for unbalanced parentheses");
    }

    #[test]
    fn test_unbalanced_parentheses_close() {
        let result = Parser::parse_sql("SELECT 1 + 2)");
        assert!(result.is_err(), "Expected error for unbalanced parentheses");
    }

    #[test]
    fn test_unbalanced_parentheses_nested() {
        let result = Parser::parse_sql("SELECT ((1 + 2)");
        assert!(
            result.is_err(),
            "Expected error for unbalanced nested parentheses"
        );
    }

    #[test]
    fn test_missing_from_keyword() {
        // "SELECT * users" is missing FROM
        let result = Parser::parse_sql("SELECT * users");
        // This might parse differently depending on implementation
        // Just ensure it doesn't panic
        let _ = result;
    }

    #[test]
    fn test_missing_select_keyword() {
        // "* FROM users" is parseable: star expression + FROM-first query
        let result = Parser::parse_sql("* FROM users");
        let _ = result;
    }

    #[test]

    fn test_incomplete_where_clause() {
        let result = Parser::parse_sql("SELECT * FROM users WHERE");
        assert!(result.is_err(), "Expected error for incomplete WHERE");
    }

    #[test]

    fn test_incomplete_and_expression() {
        let result = Parser::parse_sql("SELECT * FROM users WHERE a AND");
        assert!(result.is_err(), "Expected error for incomplete AND");
    }

    #[test]

    fn test_incomplete_or_expression() {
        let result = Parser::parse_sql("SELECT * FROM users WHERE a OR");
        assert!(result.is_err(), "Expected error for incomplete OR");
    }

    #[test]
    fn test_missing_expression_in_select() {
        let result = Parser::parse_sql("SELECT FROM users");
        // Might parse as "SELECT FROM" table named "users" or error
        let _ = result;
    }

    #[test]
    fn test_incomplete_join() {
        let result = Parser::parse_sql("SELECT * FROM users JOIN");
        assert!(result.is_err(), "Expected error for incomplete JOIN");
    }

    #[test]
    fn test_join_missing_on() {
        let result = Parser::parse_sql("SELECT * FROM users JOIN orders");
        // Some dialects allow this (CROSS JOIN), just ensure no panic
        let _ = result;
    }

    #[test]

    fn test_incomplete_order_by() {
        let result = Parser::parse_sql("SELECT * FROM users ORDER BY");
        assert!(result.is_err(), "Expected error for incomplete ORDER BY");
    }

    #[test]

    fn test_incomplete_group_by() {
        let result = Parser::parse_sql("SELECT * FROM users GROUP BY");
        assert!(result.is_err(), "Expected error for incomplete GROUP BY");
    }

    #[test]
    fn test_missing_closing_quote() {
        let result = Parser::parse_sql("SELECT 'unclosed string");
        assert!(result.is_err(), "Expected error for unclosed string");
    }

    #[test]
    fn test_invalid_operator() {
        let result = Parser::parse_sql("SELECT 1 <> 2");
        // <> is valid SQL for not equal, should parse
        let _ = result;
    }

    #[test]
    fn test_double_comma() {
        let result = Parser::parse_sql("SELECT a,, b FROM users");
        assert!(result.is_err(), "Expected error for double comma");
    }

    #[test]
    fn test_trailing_comma_in_select() {
        // Trailing comma before FROM is tolerated by the parser
        let result = Parser::parse_sql("SELECT a, b, FROM users");
        assert!(
            result.is_ok(),
            "Trailing comma before FROM should be tolerated"
        );
    }

    #[test]
    fn test_incomplete_between_low() {
        let result = Parser::parse_sql("SELECT x BETWEEN");
        assert!(result.is_err(), "Expected error for incomplete BETWEEN");
    }

    #[test]
    fn test_incomplete_between_high() {
        let result = Parser::parse_sql("SELECT x BETWEEN 1 AND");
        assert!(
            result.is_err(),
            "Expected error for incomplete BETWEEN high bound"
        );
    }

    #[test]
    fn test_incomplete_like_expression() {
        let result = Parser::parse_sql("SELECT x LIKE");
        assert!(result.is_err(), "Expected error for incomplete LIKE");
    }

    #[test]
    fn test_incomplete_ilike_expression() {
        let result = Parser::parse_sql("SELECT x ILIKE");
        assert!(result.is_err(), "Expected error for incomplete ILIKE");
    }

    #[test]
    fn test_incomplete_in_expression() {
        let result = Parser::parse_sql("SELECT x IN");
        assert!(result.is_err(), "Expected error for incomplete IN");
    }

    #[test]
    fn test_incomplete_is_distinct_from_expression() {
        let result = Parser::parse_sql("SELECT x IS DISTINCT FROM");
        assert!(
            result.is_err(),
            "Expected error for incomplete IS DISTINCT FROM"
        );
    }

    #[test]
    fn test_incomplete_bitwise_and_expression() {
        let result = Parser::parse_sql("SELECT x &");
        assert!(result.is_err(), "Expected error for incomplete bitwise AND");
    }

    #[test]
    fn test_incomplete_bitwise_or_expression() {
        let result = Parser::parse_sql("SELECT x |");
        assert!(result.is_err(), "Expected error for incomplete bitwise OR");
    }

    #[test]
    fn test_incomplete_bitwise_xor_expression() {
        let result = Parser::parse_sql("SELECT x ^");
        assert!(result.is_err(), "Expected error for incomplete bitwise XOR");
    }

    #[test]
    fn test_incomplete_bitwise_not_expression() {
        let result = Parser::parse_sql("SELECT ~");
        assert!(result.is_err(), "Expected error for incomplete bitwise NOT");
    }

    #[test]
    fn test_json_object_missing_value_rejected() {
        let result = Parser::parse_sql("SELECT JSON_OBJECT('a')");
        assert!(
            result.is_err(),
            "Expected error for JSON_OBJECT missing value"
        );
    }

    #[test]
    fn test_json_object_missing_value_after_separator_rejected() {
        let result = Parser::parse_sql("SELECT JSON_OBJECT('a':)");
        assert!(
            result.is_err(),
            "Expected error for JSON_OBJECT missing value after separator"
        );
    }

    #[test]
    fn test_unnest_requires_expression() {
        let result = parse_with_dialect("SELECT * FROM UNNEST()", DialectType::PostgreSQL);
        assert!(
            result.is_err(),
            "Expected error for UNNEST with no expressions"
        );
    }

    #[test]
    fn test_clickhouse_index_requires_expression() {
        let result = parse_with_dialect(
            "CREATE TABLE t (x Int32, INDEX idx TYPE minmax GRANULARITY 1) ENGINE = MergeTree() ORDER BY x",
            DialectType::ClickHouse,
        );
        assert!(
            result.is_err(),
            "Expected error for ClickHouse INDEX without expression"
        );
    }

    #[test]
    fn test_clickhouse_projection_index_requires_expression() {
        let result = parse_with_dialect(
            "CREATE TABLE t (x Int32, PROJECTION p INDEX TYPE minmax) ENGINE = MergeTree() ORDER BY x",
            DialectType::ClickHouse,
        );
        assert!(
            result.is_err(),
            "Expected error for ClickHouse PROJECTION INDEX without expression"
        );
    }

    #[test]
    fn test_clickhouse_partition_by_requires_expression() {
        let result = parse_with_dialect(
            "CREATE TABLE t (x Int32) ENGINE = MergeTree() PARTITION BY ORDER BY x",
            DialectType::ClickHouse,
        );
        assert!(
            result.is_err(),
            "Expected error for ClickHouse PARTITION BY without expression"
        );
    }

    #[test]
    fn test_clickhouse_ternary_requires_true_expression() {
        let result = parse_with_dialect("SELECT 1 ? : 2", DialectType::ClickHouse);
        assert!(
            result.is_err(),
            "Expected error for ClickHouse ternary without true expression"
        );
    }
}

// ============================================================================
// Empty/Whitespace Input Tests
// ============================================================================

mod empty_input {
    use super::*;

    #[test]
    fn test_empty_string() {
        let result = Parser::parse_sql("");
        // Should return empty vec or graceful error
        match result {
            Ok(stmts) => assert!(stmts.is_empty(), "Empty input should produce no statements"),
            Err(_) => {} // Also acceptable
        }
    }

    #[test]
    fn test_whitespace_only() {
        let result = Parser::parse_sql("   \n\t  ");
        match result {
            Ok(stmts) => assert!(stmts.is_empty(), "Whitespace should produce no statements"),
            Err(_) => {} // Also acceptable
        }
    }

    #[test]
    fn test_only_semicolon() {
        let result = Parser::parse_sql(";");
        // Should handle gracefully
        let _ = result;
    }

    #[test]
    fn test_multiple_semicolons() {
        let result = Parser::parse_sql(";;;");
        // Should handle gracefully
        let _ = result;
    }

    #[test]
    fn test_only_comment() {
        let result = Parser::parse_sql("-- this is a comment");
        match result {
            Ok(stmts) => assert!(stmts.is_empty(), "Comment should produce no statements"),
            Err(_) => {} // Also acceptable
        }
    }

    #[test]
    fn test_only_block_comment() {
        let result = Parser::parse_sql("/* block comment */");
        match result {
            Ok(stmts) => assert!(
                stmts.is_empty(),
                "Block comment should produce no statements"
            ),
            Err(_) => {} // Also acceptable
        }
    }
}

// ============================================================================
// Invalid Dialect Tests
// ============================================================================

mod invalid_dialect {
    use super::*;

    #[test]
    fn test_invalid_dialect_from_str() {
        let result: Result<DialectType, _> = "invalid_dialect".parse();
        assert!(result.is_err(), "Invalid dialect should return error");
    }

    #[test]
    fn test_case_insensitive_dialect() {
        let result1: Result<DialectType, _> = "POSTGRES".parse();
        let result2: Result<DialectType, _> = "PostgreSQL".parse();
        let result3: Result<DialectType, _> = "postgresql".parse();

        assert!(result1.is_ok(), "POSTGRES should be valid");
        assert!(result2.is_ok(), "PostgreSQL should be valid");
        assert!(result3.is_ok(), "postgresql should be valid");
    }

    #[test]
    fn test_alternate_dialect_names() {
        // Test alternate names for SQL Server
        let tsql1: Result<DialectType, _> = "tsql".parse();
        let tsql2: Result<DialectType, _> = "mssql".parse();
        let tsql3: Result<DialectType, _> = "sqlserver".parse();

        assert!(tsql1.is_ok(), "tsql should be valid");
        assert!(tsql2.is_ok(), "mssql should be valid");
        assert!(tsql3.is_ok(), "sqlserver should be valid");

        // Test alternate names for CockroachDB
        let crdb1: Result<DialectType, _> = "cockroachdb".parse();
        let crdb2: Result<DialectType, _> = "cockroach".parse();

        assert!(crdb1.is_ok(), "cockroachdb should be valid");
        assert!(crdb2.is_ok(), "cockroach should be valid");
    }
}

// ============================================================================
// Unicode Edge Cases
// ============================================================================

mod unicode_tests {
    use super::*;

    #[test]
    fn test_unicode_string_literal() {
        let result = Parser::parse_sql("SELECT '你好世界'");
        // Should parse Unicode string
        assert!(result.is_ok(), "Unicode string should parse: {:?}", result);
    }

    #[test]
    fn test_unicode_in_comment() {
        let result = Parser::parse_sql("SELECT 1 -- 日本語コメント");
        assert!(result.is_ok(), "Unicode in comment should parse");
    }

    #[test]
    fn test_emoji_in_string() {
        let result = Parser::parse_sql("SELECT '😀🎉'");
        // Should handle emoji in strings
        assert!(result.is_ok(), "Emoji in string should parse");
    }

    #[test]
    fn test_unicode_identifier() {
        // Unicode identifiers may or may not be supported
        let result = Parser::parse_sql("SELECT 用户名 FROM 表");
        // Just ensure it doesn't panic
        let _ = result;
    }
}

// ============================================================================
// Deeply Nested Expression Tests
// ============================================================================

mod nesting_tests {
    use super::*;

    #[test]
    fn test_deeply_nested_parentheses() {
        let sql = "SELECT ((((((((1 + 2))))))))";
        let result = Parser::parse_sql(sql);
        assert!(result.is_ok(), "Deeply nested parentheses should parse");
    }

    #[test]
    fn test_nested_subqueries() {
        let sql = "SELECT * FROM (SELECT * FROM (SELECT * FROM users))";
        let result = Parser::parse_sql(sql);
        assert!(result.is_ok(), "Nested subqueries should parse");
    }

    #[test]
    fn test_nested_case_expressions() {
        let sql = "SELECT CASE WHEN a THEN CASE WHEN b THEN 1 ELSE 2 END ELSE 3 END";
        let result = Parser::parse_sql(sql);
        assert!(result.is_ok(), "Nested CASE should parse");
    }

    #[test]
    fn test_deeply_nested_function_calls() {
        let sql = "SELECT UPPER(LOWER(TRIM(UPPER(LOWER(name)))))";
        let result = Parser::parse_sql(sql);
        assert!(result.is_ok(), "Nested functions should parse");
    }

    #[test]
    fn test_complex_nested_expression() {
        let sql = "SELECT (1 + (2 * (3 - (4 / (5 + 6)))))";
        let result = Parser::parse_sql(sql);
        assert!(result.is_ok(), "Complex nested math should parse");
    }
}

// ============================================================================
// Large/Stress Tests
// ============================================================================

mod stress_tests {
    use super::*;

    #[test]
    fn test_many_columns() {
        // Generate a SELECT with 100 columns
        let columns: Vec<String> = (1..=100).map(|i| format!("col{}", i)).collect();
        let sql = format!("SELECT {} FROM table1", columns.join(", "));
        let result = Parser::parse_sql(&sql);
        assert!(result.is_ok(), "Many columns should parse");
    }

    #[test]
    fn test_long_string_literal() {
        // Generate a long string (1KB)
        let long_string = "a".repeat(1024);
        let sql = format!("SELECT '{}'", long_string);
        let result = Parser::parse_sql(&sql);
        assert!(result.is_ok(), "Long string should parse");
    }

    #[test]
    fn test_many_unions() {
        // Generate a query with many UNIONs
        let selects: Vec<String> = (1..=20).map(|i| format!("SELECT {}", i)).collect();
        let sql = selects.join(" UNION ALL ");
        let result = Parser::parse_sql(&sql);
        assert!(result.is_ok(), "Many UNIONs should parse");
    }

    #[test]
    fn test_many_joins() {
        // Generate a query with many JOINs
        let mut sql = String::from("SELECT * FROM t1");
        for i in 2..=10 {
            sql.push_str(&format!(" JOIN t{} ON t1.id = t{}.id", i, i));
        }
        let result = Parser::parse_sql(&sql);
        assert!(result.is_ok(), "Many JOINs should parse");
    }

    #[test]
    fn test_long_in_list() {
        // Generate an IN clause with many values
        let values: Vec<String> = (1..=100).map(|i| i.to_string()).collect();
        let sql = format!("SELECT * FROM users WHERE id IN ({})", values.join(", "));
        let result = Parser::parse_sql(&sql);
        assert!(result.is_ok(), "Long IN list should parse");
    }
}

// ============================================================================
// Transpile Error Tests
// ============================================================================

mod transpile_errors {
    use super::*;

    #[test]

    fn test_transpile_invalid_sql() {
        let dialect = Dialect::get(DialectType::Generic);
        let result = dialect.transpile("SELECT (", DialectType::PostgreSQL);
        assert!(result.is_err(), "Transpiling invalid SQL should fail");
    }

    #[test]
    fn test_transpile_empty_input() {
        let dialect = Dialect::get(DialectType::Generic);
        let result = dialect.transpile("", DialectType::PostgreSQL);
        match result {
            Ok(stmts) => assert!(stmts.is_empty(), "Empty input should transpile to empty"),
            Err(_) => {} // Also acceptable
        }
    }

    #[test]
    fn test_transpile_same_dialect() {
        let dialect = Dialect::get(DialectType::PostgreSQL);
        let result = dialect.transpile("SELECT 1", DialectType::PostgreSQL);
        assert!(result.is_ok(), "Same dialect transpile should work");
        assert_eq!(result.unwrap()[0], "SELECT 1");
    }
}

// ============================================================================
// Generator Error Tests
// ============================================================================

mod generator_tests {
    use super::*;

    #[test]
    fn test_generator_valid_ast() {
        let dialect = Dialect::get(DialectType::Generic);
        let ast = dialect.parse("SELECT 1").expect("Parse failed");
        let result = Generator::sql(&ast[0]);
        assert!(result.is_ok(), "Generator should produce valid SQL");
    }

    #[test]
    fn test_roundtrip_consistency() {
        // Parse, generate, parse again should be consistent
        let sql = "SELECT a, b FROM users WHERE id = 1";
        let ast1 = Parser::parse_sql(sql).expect("First parse failed");
        let generated = Generator::sql(&ast1[0]).expect("Generate failed");
        let ast2 = Parser::parse_sql(&generated).expect("Second parse failed");
        let generated2 = Generator::sql(&ast2[0]).expect("Second generate failed");

        assert_eq!(generated, generated2, "Roundtrip should be stable");
    }
}

// ============================================================================
// Truncated Input Tests
// ============================================================================

mod truncated_input {
    use super::*;
    use std::sync::mpsc;
    use std::time::Duration;

    /// Parse on a worker thread, so a parse that never returns fails the test instead of
    /// hanging the suite. Returns `false` if the parse was still running after the timeout.
    fn parse_terminates(sql: &str) -> bool {
        let owned = sql.to_string();
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let _ = tx.send(Parser::parse_sql(&owned).is_ok());
        });
        rx.recv_timeout(Duration::from_secs(10)).is_ok()
    }

    /// Statements that exercise the parser's closer-scanning loops: parenthesised data type
    /// arguments, function arguments, index columns, table hints, star modifiers, JSON paths,
    /// subscripts, window specs and privilege lists.
    const STATEMENTS: &[&str] = &[
        "CREATE TABLE t (a VARCHAR2(2328 CHAR), b DATETIME2(2))",
        "SELECT CAST(x AS UserDefinedType(1, 2 CHAR)) FROM t",
        "ALTER TABLE t ADD COLUMN c NVARCHAR2(100)",
        "SELECT a.:CustomType(1, 2) FROM t",
        "SELECT JSON_VALUE(a, '$.b' RETURNING VARCHAR2(10)) FROM t",
        "SELECT x::DECIMAL(10, 2) FROM t",
        "SELECT CAST(x AS DECIMAL(10, 2)) FROM t",
        "SELECT f(a, b, c) FROM t",
        "SELECT * EXCEPT (a, b) FROM t",
        "SELECT * FROM t WITH (NOLOCK, INDEX(i))",
        "SELECT a[1], b:c FROM t",
        "SELECT COUNT(*) OVER (PARTITION BY a ORDER BY b) FROM t",
        "CREATE INDEX i ON t (a ASC, b DESC)",
        "CREATE FUNCTION f(a INT, b TEXT) RETURNS INT",
        "GRANT SELECT, INSERT ON t TO u",
        "INSERT INTO t (a, b) VALUES (1, 2)",
    ];

    /// The six-character case that motivated this: a custom parenthesised data type reached
    /// through `.:`, with no closing paren. `parse_data_type`'s argument loop broke only on
    /// `RParen`, which is false at end of input, while `advance` past the end returns the last
    /// token without moving the cursor — so the loop appended that token to the same `String`
    /// forever.
    #[test]
    fn test_truncated_custom_type_args_terminate() {
        for sql in ["a.:S1(", "CAST(x AS S1(", "CREATE TABLE t (c S1("] {
            assert!(parse_terminates(sql), "Parsing did not terminate: {sql:?}");
            assert!(
                Parser::parse_sql(sql).is_err(),
                "Expected error for truncated custom type args: {sql:?}"
            );
        }
    }

    /// General oracle for the whole class: every truncated prefix of a valid statement must
    /// terminate. This does not need to know which loops exist, so it also covers the
    /// closer-scanning loops nobody enumerated.
    #[test]
    fn test_every_truncated_prefix_terminates() {
        for sql in STATEMENTS {
            for end in 1..=sql.len() {
                if !sql.is_char_boundary(end) {
                    continue;
                }
                let prefix = &sql[..end];
                assert!(
                    parse_terminates(prefix),
                    "Parsing did not terminate for prefix {prefix:?} of {sql:?}"
                );
            }
        }
    }
}
