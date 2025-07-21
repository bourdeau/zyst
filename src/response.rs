use crate::errors::ZystError;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ZystResponse {
    Ok,                   // "OK"
    Int(i64),             // "(integer) 123"
    SimpleString(String), // "foo"
    List(Vec<String>),    // "1) foo\n2) bar\n"
    Nil,                  // "(nil)"
    EmptyArray,           // "(empty array)"
    Error(ZystError),     // Handles errors gracefully
}

impl fmt::Display for ZystResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZystResponse::Ok => write!(f, "+OK\r\n"),
            ZystResponse::Int(value) => write!(f, "+(integer) {value}\r\n"),
            ZystResponse::SimpleString(value) => write!(f, "+{value}\r\n"),
            ZystResponse::List(values) => {
                let mut response = format!("*{}\r\n", values.len());

                for value in values {
                    response.push_str(&format!("${}\r\n{}\r\n", value.len(), value));
                }
                write!(f, "{response}")
            }
            ZystResponse::Nil => write!(f, "+(nil)\r\n"),
            ZystResponse::EmptyArray => write!(f, "+(empty array)\r\n"),
            ZystResponse::Error(err) => write!(f, "-{err}\r\n"),
        }
    }
}
