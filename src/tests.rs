use crate::JsonObject;

use super::{parse, Value::*};
use std::collections::HashMap;

#[test]
fn null() {
    assert_eq!(parse("null"), Ok(Null));
}

#[test]
fn null_err() {
    assert_eq!(parse("nulz"), Err("expected null".to_string()));
}

#[test]
fn bool() {
    assert_eq!(parse("true"), Ok(Bool(true)));
    assert_eq!(parse("false"), Ok(Bool(false)));
}

#[test]
fn number() {
    assert_eq!(parse("42"), Ok(Number(42.0)));
    assert_eq!(parse("42.42"), Ok(Number(42.42)));
    assert_eq!(parse("-42"), Ok(Number(-42.0)));
    assert_eq!(parse("+42"), Ok(Number(42.0)));
    assert_eq!(parse("1e-005"), Ok(Number(0.00001)))
}

#[test]
fn number_trailing_spaces() {
    assert_eq!(parse("42  "), Ok(Number(42.0)));
}

#[test]
fn string() {
    assert_eq!(
        parse(r#""test string""#),
        Ok(String("test string".to_string()))
    )
}

#[test]
fn string_escape() {
    assert_eq!(
        parse(r#" "\"test string\"" "#),
        Ok(String(r#""test string""#.to_string()))
    )
}

#[test]
fn string_escape_at_end_of_text() {
    assert_eq!(
        parse(r#""err\"#),
        Err("no char to escape".to_string())
    )
}

#[test]
fn string_err() {
    assert_eq!(parse(r#""broken"#), Err("invalid json string".to_string()))
}

#[test]
fn array() {
    assert_eq!(
        parse("[null, true, false, 42.42, \"this is a string\"]"),
        Ok(Array(vec![
            Null,
            Bool(true),
            Bool(false),
            Number(42.42),
            String("this is a string".to_string()),
        ]))
    );
}

#[test]
fn object() {
    let json = r#"{
            "boolean": false,
            "text": "text value",
            "number": 42.42
        }"#;
    assert_eq!(
        parse(json),
        Ok(Object({
            let mut map = HashMap::new();
            map.insert("boolean".to_string(), Bool(false));
            map.insert("text".to_string(), String("text value".to_string()));
            map.insert("number".to_string(), Number(42.42));
            JsonObject(map)
        }))
    );
}

#[test]
fn object_with_nested_array() {
    let json = r#"{
            "array": [
                true,
                false,
                "hello"]
        }"#;
    assert_eq!(
        parse(json),
        Ok(Object({
            let mut map = HashMap::new();
            map.insert(
                "array".to_string(),
                Array(vec![Bool(true), Bool(false), String("hello".to_string())]),
            );
            JsonObject(map)
        }))
    );
}

#[test]
fn nesting() {
    let json = r#"{
            "array": [
                true,
                false,
                {
                    "text": "this is a string",
                    "nested array": [
                        null,
                        false,
                        true
                    ]
                }]
        }"#;
    assert_eq!(
        parse(json),
        Ok(Object({
            let mut map = HashMap::new();
            map.insert(
                "array".to_string(),
                Array(vec![
                    Bool(true),
                    Bool(false),
                    Object({
                        let mut map = HashMap::new();
                        map.insert("text".to_string(), String("this is a string".to_string()));
                        map.insert(
                            "nested array".to_string(),
                            Array(vec![Null, Bool(false), Bool(true)]),
                        );
                        JsonObject(map)
                    }),
                ]),
            );
            JsonObject(map)
        }))
    );
}

#[test]
fn unexpected_text_after() {
    let json = "[null] invalid";
    assert_eq!(parse(json), Err("unexpected text after value".to_string()))
}
