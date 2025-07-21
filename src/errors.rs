use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ZystError {
    #[error("Invalid command")]
    InvalidCommand,
    #[error("(nil)")]
    Nil,
    #[error("(empty array)")]
    EmptyArray,
    #[error("ERR WRONGTYPE Operation against a key holding the wrong kind of value")]
    WrongType,
    #[error("ERR value is not an integer")]
    NotInt,
    #[error("value is not an integer or out of range")]
    NotIntOrOutOfRange,
    #[error("ERR unexpected database error")]
    DatabaseError,
    #[error("ERR regex error")]
    RegexError,
    #[error("Error: TTL is required")]
    TTL,
    #[error("{0}")]
    Custom(String),
    #[error("Wrong number of argument")]
    WrongNumberArgs,

    // RESP Parsing Errors
    #[error("ERR Protocol error: empty request")]
    EmptyRequest,
    #[error("ERR Protocol error: expected '*', got something else")]
    InvalidArrayPrefix,
    #[error("ERR Protocol error: invalid array length")]
    InvalidArrayLength,
    #[error("ERR Protocol error: expected '$', got something else")]
    InvalidBulkStringPrefix,
    #[error("ERR Protocol error: wrong number of elements")]
    WrongElementCount,
}

pub fn format_redis_error(error: ZystError) -> String {
    format!("-{error}\r\n")
}
