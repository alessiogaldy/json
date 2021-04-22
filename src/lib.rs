#[cfg(test)]
mod tests;

use core::f64;
use std::{collections::HashMap, iter::Peekable, str::Chars};

struct Reader<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Reader<'a> {
    fn new(raw: &'a str) -> Self {
        Self {
            chars: raw.chars().peekable(),
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn next(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn skip_whitespaces(&mut self) -> bool {
        loop {
            match self.peek() {
                Some(c) if c.is_whitespace() => {
                    self.next();
                }
                None => return false,
                _ => return true,
            }
        }
    }

    fn read_until(&mut self, delimiters: &Vec<char>) -> Option<(String, char)> {
        let (value, matched) = self.read_until_or_end(delimiters);
        if matched.is_some() {
            self.next().unwrap();
        }
        matched.map(|c| (value, c))
    }

    fn read_until_or_end(&mut self, delimiters: &Vec<char>) -> (String, Option<char>) {
        let mut result = String::new();
        while let Some(c) = self.peek() {
            if delimiters.contains(c) {
                return (result, Some(c.to_owned()));
            }
            result.push(self.next().unwrap())
        }
        (result, None)
    }

    fn skip_until(&mut self, delimiters: &Vec<char>) -> Option<char> {
        self.read_until(delimiters).map(|(_, c)| c)
    }

    fn read_token(&mut self, token: &str) -> bool {
        for c in token.chars() {
            if self.next() != Some(c) {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

fn parse_array(reader: &mut Reader) -> Result<Vec<Value>, String> {
    reader.next().unwrap();
    if !reader.skip_whitespaces() {
        return Err("unable to parse array".to_string());
    }
    if reader.peek() == Some(&']') {
        return Ok(Vec::new());
    }
    let mut values = Vec::new();
    loop {
        values.push(parse_value(reader)?);
        if let Some(c) = reader.skip_until(&vec![',', ']']) {
            if c == ']' {
                return Ok(values);
            }
        } else {
            return Err("unable to parse array".to_string());
        }
    }
}

fn parse_string(reader: &mut Reader) -> Result<String, String> {
    match reader.read_until(&vec!['"']) {
        Some((value, _)) => Ok(value),
        None => Err("invalid json string".to_string()),
    }
}

fn parse_null(reader: &mut Reader) -> Result<Value, String> {
    if reader.read_token("null") {
        Ok(Value::Null)
    } else {
        Err("expected null".to_string())
    }
}

fn parse_true(reader: &mut Reader) -> Result<Value, String> {
    if reader.read_token("true") {
        Ok(Value::Bool(true))
    } else {
        Err("expected true".to_string())
    }
}

fn parse_false(reader: &mut Reader) -> Result<Value, String> {
    if reader.read_token("false") {
        Ok(Value::Bool(false))
    } else {
        Err("expected false".to_string())
    }
}

fn parse_object(reader: &mut Reader) -> Result<HashMap<String, Value>, String> {
    reader.next().unwrap();
    let mut value = HashMap::new();

    while let Some(delimiter) = reader.skip_until(&vec!['"', '}']) {
        if delimiter == '}' {
            return Ok(value);
        }
        let name = parse_string(reader)?;
        if reader.skip_until(&vec![':']).is_none() {
            return Err("missing property value".to_string());
        }
        value.insert(name, parse_value(reader)?);

        if let Some(delimiter) = reader.skip_until(&vec![',', '}']) {
            if delimiter == '}' {
                return Ok(value);
            }
        } else {
            return Err("missing property value".to_string());
        }
    }

    Err("invalid json object".to_string())
}

fn parse_number(reader: &mut Reader) -> Result<f64, String> {
    let (raw, _) = reader.read_until_or_end(&vec![',', ']', '}']);
    raw.parse()
        .map_err(|_| format!("{} is not a valid number", raw))
}

fn parse_value(reader: &mut Reader) -> Result<Value, String> {
    if !reader.skip_whitespaces() {
        return Err("empty string".to_string());
    }
    return match reader.peek() {
        Some('n') => parse_null(reader),
        Some('t') => parse_true(reader),
        Some('f') => parse_false(reader),
        Some('[') => parse_array(reader).map(Value::Array),
        Some('"') => {
            reader.next().unwrap();
            parse_string(reader).map(Value::String)
        }
        Some('{') => parse_object(reader).map(Value::Object),
        Some(c) if *c == '+' || *c == '-' || c.is_digit(10) => {
            parse_number(reader).map(Value::Number)
        }
        _ => Err("malformed json".to_string()),
    };
}

pub fn parse(raw: &str) -> Result<Value, String> {
    let reader = &mut Reader::new(raw);
    let value = parse_value(reader)?;
    if reader.skip_whitespaces() {
        return Err("unexpected text after value".to_string());
    }
    Ok(value)
}
