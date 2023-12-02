#![allow(dead_code)]

#[derive(Debug, PartialEq)]
pub enum ValueType {
    SimpleString,
    Error,
    Integer,
    Null,
    BulkString,
    Array,
}

#[derive(Debug)]
pub struct Value {
    pub value: Option<String>,
    pub value_type: ValueType,
    pub null: bool,
    pub array: Vec<Value>,
}

pub struct Parser {
    cursor: usize,
    buf: String,
}
pub fn stringify(value: &Value) -> String {
    let mut result = String::new();

    // if value type is array then we need to recurse
    // else we can directly append values to the result
    match value.value_type {
        ValueType::Array => {
            result += format!("*{}\r\n", value.array.len()).as_str();
            for v in value.array.iter() {
                let val = stringify(v);
                result += &val;
            }
        }
        ValueType::SimpleString => {
            result += format!("+{}", value.value.clone().unwrap()).as_str();
            result.push_str("\r\n");
        }
        ValueType::Null => {
            result += "$-1\r\n";
        }
        ValueType::Integer => {
            result += format!(":{}\r\n", value.value.clone().unwrap()).as_str();
        }
        ValueType::BulkString => {
            result += format!(
                "${}\r\n{}\r\n",
                value.value.clone().unwrap().len(),
                value.value.clone().unwrap()
            )
            .as_str();
        }
        ValueType::Error => {
            result += format!("-{}\r\n", value.value.clone().unwrap()).as_str();
        }
    }

    result
}

impl Parser {
    pub fn new(input: String) -> Self {
        Parser {
            cursor: 0,
            buf: input,
        }
    }

    /// Iterate over next character
    /// it doesn't check if cursor > buf.len(), be careful!
    pub fn next(&mut self) -> char {
        if self.cursor >= self.buf.len() {
            panic!("Cannot parse input!");
        }
        let mut chars = self.buf.chars();
        let ch = chars.nth(self.cursor).unwrap();
        self.cursor += 1;
        ch
    }

    /// Return next `command` String til `\r\n`
    pub fn next_command(&mut self) -> String {
        let mut command = String::new();
        let mut ch = self.next();
        let mut found_carriage = false;
        while ch != '\n' {
            if ch == '\r' {
                found_carriage = true;
            } else {
                command.push(ch);
            }
            ch = self.next();
        }

        // if command terminator is found correctly
        // else return an empty string
        if found_carriage && ch == '\n' {
            command
        } else {
            String::new()
        }
    }

    pub fn parse(&mut self) -> Value {
        let first_char = self.next();
        // dbg!(first_char.clone());

        match first_char {
            '+' => {
                let string = self.next_command();
                Value {
                    null: false,
                    value: Some(string),
                    value_type: ValueType::SimpleString,
                    array: Vec::new(),
                }
            }
            '-' => {
                let string = self.next_command();
                Value {
                    value: Some(string),
                    value_type: ValueType::Error,
                    array: Vec::new(),
                    null: false,
                }
            }
            ':' => {
                let integer_str = self.next_command();
                Value {
                    value: Some(integer_str),
                    value_type: ValueType::Integer,
                    array: Vec::new(),
                    null: false,
                }
            }
            '$' => {
                let string = self.next_command();
                match string.as_str() {
                    "-1" => Value {
                        value: None,
                        value_type: ValueType::Null,
                        null: true,
                        array: Vec::new(),
                    },
                    _ => {
                        let len = string.parse::<usize>().unwrap_or(0);
                        // $<length>\r\n<data>\r\n
                        let mut string = self.next_command();
                        string = string.get(0..len).unwrap().to_string();
                        Value {
                            value: Some(string),
                            value_type: ValueType::BulkString,
                            null: false,
                            array: Vec::new(),
                        }
                    }
                }
            }
            '*' => {
                // if `buf` is an array
                // *<number-of-elements>\r\n<element-1>...<element-n>
                let len = self.next_command().parse::<usize>().unwrap_or(0);
                //<element-1>...<element-n>
                let mut arr: Vec<Value> = Vec::with_capacity(len);
                for _ in 0..len {
                    let v = self.parse();
                    arr.push(v);
                }

                Value {
                    value: None,
                    value_type: ValueType::Array,
                    array: arr,
                    null: false,
                }
            }
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_array_parse_ping_array() {
        let input = "*1\r\n$4\r\nping\r\n".to_string();
        let mut p = Parser::new(input);
        let val = p.parse();
        dbg!(val);
    }

    #[test]
    fn test_array_two_elements_parse_array() {
        let input = "*2\r\n$3\r\nget\r\n$3\r\nkey\r\n".to_string();
        let mut p = Parser::new(input);
        let val = p.parse();
        dbg!(val);
    }

    #[test]
    fn test_array_two_elements_parse_respect_length_array() {
        let input = "*2\r\n$4\r\necho\r\n$5\r\nhello world\r\n".to_string();
        let mut p = Parser::new(input);
        let val = p.parse();
        dbg!(val);
    }

    #[test]
    fn test_empty_bulk_string() {
        let input = "$0\r\n\r\n".to_string();
        let mut p = Parser::new(input);
        let val = p.parse();
        dbg!(val);
    }

    #[test]
    fn test_null_case() {
        let input = "$-1\r\n".to_string();
        let mut p = Parser::new(input);
        let val = p.parse();
        dbg!(val);
    }

    #[test]
    fn test_simple_string() {
        let input = "+OK\r\n".to_string();
        let mut p = Parser::new(input);
        let val = p.parse();
        dbg!(val);
    }

    #[test]
    fn test_simple_string_hello_world() {
        let input = "+hello world\r\n".to_string();
        let mut p = Parser::new(input);
        let val = p.parse();
        dbg!(val);
    }

    #[test]
    fn test_basic_error_message() {
        let input = "-Error message\r\n".to_string();
        let mut p = Parser::new(input);
        let val = p.parse();
        dbg!(val);
    }

    #[test]
    fn test_negative_integer() {
        let input = ":-333\r\n".to_string();
        let mut p = Parser::new(input);
        let val = p.parse();
        dbg!(val);
    }

    #[test]
    fn test_postive_integer() {
        let input = ":89\r\n".to_string();
        let mut p = Parser::new(input);
        let val = p.parse();
        dbg!(val);
    }

    #[test]
    fn test_mixed_array() {
        let input = "*4\r\n$5\r\nhello\r\n:-33\r\n:69\r\n+MIXED\r\n".to_string();
        let mut p = Parser::new(input);
        let val = p.parse();
        dbg!(val);
    }

    #[test]
    #[should_panic]
    fn test_invalid_mixed_array() {
        let input = "*5\r\n$5\r\nhello\r\n:-33\r\n:69\r\n+MIXED\r\n".to_string();
        let mut p = Parser::new(input);
        let val = p.parse();
        dbg!(val);
    }

    #[test]
    fn test_simple_string_stringify() {
        let val = Value {
            value: Some("Hello".to_string()),
            value_type: ValueType::SimpleString,
            null: false,
            array: Vec::new(),
        };

        let s = stringify(&val);
        dbg!(s);
    }

    #[test]
    fn test_null_stringify() {
        let val = Value {
            value: None,
            value_type: ValueType::Null,
            null: true,
            array: Vec::new(),
        };

        let s = stringify(&val);
        dbg!(s);
    }

    #[test]
    fn test_simple_string_array_stringify() {
        let val = Value {
            value: None,
            value_type: ValueType::Array,
            null: false,
            array: vec![
                Value {
                    value: Some("Hello".to_string()),
                    value_type: ValueType::SimpleString,
                    null: false,
                    array: Vec::new(),
                },
                Value {
                    value: Some("World".to_string()),
                    value_type: ValueType::SimpleString,
                    null: false,
                    array: Vec::new(),
                },
                Value {
                    value: Some("John".to_string()),
                    value_type: ValueType::SimpleString,
                    null: false,
                    array: Vec::new(),
                },
            ],
        };

        let s = stringify(&val);
        dbg!(s);
    }

    #[test]
    fn test_bulk_string() {
        let val = Value {
            value: Some("Hello".to_string()),
            value_type: ValueType::BulkString,
            null: false,
            array: Vec::new(),
        };
        let s = stringify(&val);
        dbg!(s);
    }
}
