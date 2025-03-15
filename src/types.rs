use indexmap::IndexMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type Db = Arc<RwLock<IndexMap<String, DbValue>>>;

#[derive(Debug, Clone)]
pub struct Command {
    pub command_type: CommandType,
    pub args: CommandArgs,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum CommandType {
    DOCS,
    PONG,
    GET,
    SET,
    DEL,
    FLUSHDB,
    KEYS,
    EXISTS,
    EXPIRE,
    TTL,
    INCR,
    DECR,
    INCRBY,
    LPUSH,
    LRANGE,
    RPUSH,
    LPOP,
    RPOP,
    HSET,
    HGET,
    HGETALL,
    HDEL,
    CLIENT,
    SADD,
    SMEMBERS,
    SREM,
}

#[derive(Debug, Clone)]
pub enum CommandArgs {
    NoArgs,                    // PONG, FLUSHDB
    SingleKey(String),         // GET key
    MultipleKeys(Vec<String>), // DEL key1 key2 key3
    KeyWithValue {
        key: String,
        value: String,
    }, // SET key value
    KeyWithValues {
        key: String,
        values: Vec<String>,
    },
    HashFields {
        key: String,
        fields: IndexMap<String, String>,
    }, // HSET key field1 value1 field2 value2
}

#[derive(Debug, Clone, Default)]
pub struct KeyBase<T> {
    pub name: String,
    pub data: T,
    pub expires_at: Option<i64>,
}

pub type Key = KeyBase<Option<String>>;
pub type KeyList = KeyBase<VecDeque<String>>;
pub type KeySet = KeyBase<HashSet<String>>;
pub type KeyHash = KeyBase<IndexMap<String, String>>;

#[derive(Debug, Clone)]
pub enum DbValue {
    StringKey(Key),
    ListKey(KeyList),
    SetKey(KeySet),
    HashKey(KeyHash),
}

#[derive(Debug, Clone, Copy)]
pub enum ListPushType {
    LPUSH,
    RPUSH,
}

#[derive(Debug, Clone, Copy)]
pub enum PopType {
    LPOP,
    RPOP,
}
