use crate::parser::parse_command;
use crate::types::CommandType;
use crate::types::Db;

use crate::commands::db::*;
use crate::commands::hashsets::*;
use crate::commands::keys::*;
use crate::commands::lists::*;
use crate::commands::misc::*;
use crate::commands::sets::*;
use crate::errors::ZystError;
use crate::response::ZystResponse;

pub async fn process_command(
    command: Vec<String>,
    db: &Db,
    restore: bool,
) -> Result<ZystResponse, ZystError> {
    let command = parse_command(command, restore).await?;

    match command.command_type {
        CommandType::DOCS => docs().await,
        CommandType::PONG => pong().await,
        CommandType::GET => get_key(db, command).await,
        CommandType::SET => set_key(db, command).await,
        CommandType::DEL => delete_key(db, command).await,
        CommandType::FLUSHDB => flush_db(db).await,
        CommandType::KEYS => get_keys(db, command).await,
        CommandType::EXISTS => exists(db, command).await,
        CommandType::EXPIRE => expire(db, command).await,
        CommandType::TTL => ttl(db, command).await,
        CommandType::INCR => incr(db, command).await,
        CommandType::DECR => decr(db, command).await,
        CommandType::INCRBY => incrby(db, command).await,
        CommandType::LPUSH => lpush(db, command).await,
        CommandType::LRANGE => lrange(db, command).await,
        CommandType::RPUSH => rpush(db, command).await,
        CommandType::LPOP => lpop(db, command).await,
        CommandType::RPOP => rpop(db, command).await,
        CommandType::HSET => hset(db, command).await,
        CommandType::HGET => hget(db, command).await,
        CommandType::HGETALL => hgetall(db, command).await,
        CommandType::HDEL => hdel(db, command).await,
        CommandType::CLIENT => client().await,
        CommandType::SADD => sadd(db, command).await,
        CommandType::SMEMBERS => smembers(db, command).await,
        CommandType::SREM => srem(db, command).await,
    }
}
