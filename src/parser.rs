use crate::aof::write_aof;
use crate::commands::build::*;
use crate::errors::ZystError;
use crate::types::Command;

pub async fn parse_command(mut args: Vec<String>, restore: bool) -> Result<Command, ZystError> {
    if args.is_empty() {
        return Err(ZystError::InvalidCommand);
    }

    let command_type = args[0].to_uppercase();
    args.remove(0);

    let command = match command_type.as_str() {
        "DOCS" => build_docs_command(),
        "PING" => build_pong_command(),
        "FLUSHDB" => build_flush_db_command(),
        "GET" => build_get_command(&args),
        "SET" => build_set_command(&args),
        "DEL" => build_delete_command(&args),
        "KEYS" => build_keys_command(&args),
        "EXISTS" => build_exists_command(&args),
        "EXPIRE" => build_expire_command(&args),
        "TTL" => build_ttl_command(&args),
        "INCR" => build_incr_command(&args),
        "DECR" => build_decr_command(&args),
        "INCRBY" => build_incrby_command(&args),
        "LPUSH" => build_lpush_command(&args),
        "RPUSH" => build_rpush_command(&args),
        "LRANGE" => build_lrange_command(&args),
        "LPOP" => build_lpop_command(&args),
        "RPOP" => build_rpop_command(&args),
        "HSET" => build_hset_command(&args),
        "HGET" => build_hget_command(&args),
        "HGETALL" => build_hgetall_command(&args),
        "HDEL" => build_hdel_command(&args),
        "CLIENT" => build_client_command(&args),
        "SADD" => build_sadd_command(&args),
        "SMEMBERS" => build_smembers_command(&args),
        "SREM" => build_srem_command(&args),
        _ => return Err(ZystError::InvalidCommand),
    }?;

    if !restore {
        write_aof(&command)
            .await
            .expect("Error writing to AOF file!");
    }

    Ok(command)
}
