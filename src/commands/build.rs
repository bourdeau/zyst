use crate::errors::ZystError;
use crate::types::{Command, CommandArgs, CommandType};
use indexmap::IndexMap;

pub fn build_docs_command() -> Result<Command, ZystError> {
    Ok(Command {
        command_type: CommandType::DOCS,
        args: CommandArgs::NoArgs,
    })
}

pub fn build_pong_command() -> Result<Command, ZystError> {
    Ok(Command {
        command_type: CommandType::PONG,
        args: CommandArgs::NoArgs,
    })
}

pub fn build_flush_db_command() -> Result<Command, ZystError> {
    Ok(Command {
        command_type: CommandType::FLUSHDB,
        args: CommandArgs::NoArgs,
    })
}

pub fn build_get_command(args: &[String]) -> Result<Command, ZystError> {
    if args.is_empty() {
        return Err(ZystError::WrongNumberArgs);
    }
    Ok(Command {
        command_type: CommandType::GET,
        args: CommandArgs::SingleKey(args[0].to_string()),
    })
}

pub fn build_keys_command(args: &[String]) -> Result<Command, ZystError> {
    if args.is_empty() {
        return Err(ZystError::WrongNumberArgs);
    }
    Ok(Command {
        command_type: CommandType::KEYS,
        args: CommandArgs::SingleKey(args[0].to_string()),
    })
}

pub fn build_set_command(args: &[String]) -> Result<Command, ZystError> {
    if args.len() != 2 {
        return Err(ZystError::WrongNumberArgs);
    }
    Ok(Command {
        command_type: CommandType::SET,
        args: CommandArgs::KeyWithValue {
            key: args[0].to_string(),
            value: args[1].to_string(),
        },
    })
}

pub fn build_delete_command(args: &[String]) -> Result<Command, ZystError> {
    if args.is_empty() {
        return Err(ZystError::WrongNumberArgs);
    }
    Ok(Command {
        command_type: CommandType::DEL,
        args: CommandArgs::MultipleKeys(args.to_vec()),
    })
}

pub fn build_exists_command(args: &[String]) -> Result<Command, ZystError> {
    if args.is_empty() {
        return Err(ZystError::WrongNumberArgs);
    }
    Ok(Command {
        command_type: CommandType::EXISTS,
        args: CommandArgs::MultipleKeys(args.to_vec()),
    })
}

pub fn build_expire_command(args: &[String]) -> Result<Command, ZystError> {
    if args.len() < 2 {
        return Err(ZystError::WrongNumberArgs);
    }

    Ok(Command {
        command_type: CommandType::EXPIRE,
        args: CommandArgs::KeyWithValue {
            key: args[0].to_string(),
            value: args[1].to_string(),
        },
    })
}

pub fn build_ttl_command(args: &[String]) -> Result<Command, ZystError> {
    if args.is_empty() {
        return Err(ZystError::WrongNumberArgs);
    }
    Ok(Command {
        command_type: CommandType::TTL,
        args: CommandArgs::SingleKey(args[0].to_string()),
    })
}

pub fn build_incr_command(args: &[String]) -> Result<Command, ZystError> {
    if args.is_empty() {
        return Err(ZystError::WrongNumberArgs);
    }
    Ok(Command {
        command_type: CommandType::INCR,
        args: CommandArgs::SingleKey(args[0].to_string()),
    })
}

pub fn build_decr_command(args: &[String]) -> Result<Command, ZystError> {
    if args.is_empty() {
        return Err(ZystError::WrongNumberArgs);
    }
    Ok(Command {
        command_type: CommandType::DECR,
        args: CommandArgs::SingleKey(args[0].to_string()),
    })
}

pub fn build_incrby_command(args: &[String]) -> Result<Command, ZystError> {
    if args.len() < 2 {
        return Err(ZystError::WrongNumberArgs);
    }

    Ok(Command {
        command_type: CommandType::INCRBY,
        args: CommandArgs::KeyWithValue {
            key: args[0].to_string(),
            value: args[1].to_string(),
        },
    })
}

fn build_push_command(args: &[String], cmd_type: CommandType) -> Result<Command, ZystError> {
    Ok(Command {
        command_type: cmd_type,
        args: CommandArgs::KeyWithValues {
            key: args[0].to_string(),
            values: args.iter().skip(1).cloned().collect::<Vec<String>>(),
        },
    })
}

pub fn build_lpush_command(args: &[String]) -> Result<Command, ZystError> {
    build_push_command(args, CommandType::LPUSH)
}

pub fn build_rpush_command(args: &[String]) -> Result<Command, ZystError> {
    build_push_command(args, CommandType::RPUSH)
}

pub fn build_lrange_command(args: &[String]) -> Result<Command, ZystError> {
    if args.len() < 3 {
        return Err(ZystError::WrongNumberArgs);
    }

    Ok(Command {
        command_type: CommandType::LRANGE,
        args: CommandArgs::KeyWithValues {
            key: args[0].to_string(),
            values: vec![args[1].to_string(), args[2].to_string()],
        },
    })
}

pub fn build_lpop_command(args: &[String]) -> Result<Command, ZystError> {
    build_lpop_rpop_command(args, CommandType::LPOP)
}
pub fn build_rpop_command(args: &[String]) -> Result<Command, ZystError> {
    build_lpop_rpop_command(args, CommandType::RPOP)
}

fn build_lpop_rpop_command(
    args: &[String],
    cmd_type: CommandType,
) -> Result<Command, ZystError> {
    if args.is_empty() {
        return Err(ZystError::WrongNumberArgs);
    }

    let command = if args.len() == 1 {
        Command {
            command_type: cmd_type,
            args: CommandArgs::SingleKey(args[0].clone()),
        }
    } else {
        Command {
            command_type: cmd_type,
            args: CommandArgs::KeyWithValue {
                key: args[0].clone(),
                value: args[1].clone(),
            },
        }
    };

    Ok(command)
}

pub fn build_hset_command(args: &[String]) -> Result<Command, ZystError> {
    if args.len() < 3 || args.len() % 2 == 0 {
        return Err(ZystError::WrongNumberArgs);
    }

    let key = args[0].clone();
    let fields = args[1..]
        .iter()
        .step_by(2) // Selects every other element starting from the first (field)
        .zip(args[2..].iter().step_by(2)) // Pairs each field with the next value. zip() is great!
        .map(|(field, value)| (field.clone(), value.clone())) // Convert to owned Strings
        .collect::<IndexMap<String, String>>();

    Ok(Command {
        command_type: CommandType::HSET,
        args: CommandArgs::HashFields { key, fields },
    })
}

pub fn build_hget_command(args: &[String]) -> Result<Command, ZystError> {
    if args.len() != 2 {
        return Err(ZystError::WrongNumberArgs);
    }

    Ok(Command {
        command_type: CommandType::HGET,
        args: CommandArgs::KeyWithValue {
            key: args[0].to_string(),
            value: args[1].to_string(),
        },
    })
}

pub fn build_hgetall_command(args: &[String]) -> Result<Command, ZystError> {
    if args.is_empty() {
        return Err(ZystError::WrongNumberArgs);
    }

    Ok(Command {
        command_type: CommandType::HGETALL,
        args: CommandArgs::SingleKey(args[0].to_string()),
    })
}

pub fn build_hdel_command(args: &[String]) -> Result<Command, ZystError> {
    if args.len() < 2 {
        return Err(ZystError::WrongNumberArgs);
    }

    Ok(Command {
        command_type: CommandType::HDEL,
        args: CommandArgs::KeyWithValues {
            key: args[0].clone(),
            values: args.iter().skip(1).cloned().collect::<Vec<String>>(),
        },
    })
}

pub fn build_client_command(args: &[String]) -> Result<Command, ZystError> {
    if args.len() < 2 {
        return Err(ZystError::WrongNumberArgs);
    }

    Ok(Command {
        command_type: CommandType::CLIENT,
        args: CommandArgs::KeyWithValues {
            key: args[0].clone(),
            values: args.iter().skip(1).cloned().collect::<Vec<String>>(),
        },
    })
}

pub fn build_sadd_command(args: &[String]) -> Result<Command, ZystError> {
    if args.len() < 2 {
        return Err(ZystError::WrongNumberArgs);
    }

    Ok(Command {
        command_type: CommandType::SADD,
        args: CommandArgs::KeyWithValues {
            key: args[0].to_string(),
            values: args.iter().skip(1).cloned().collect::<Vec<String>>(),
        },
    })
}

pub fn build_smembers_command(args: &[String]) -> Result<Command, ZystError> {
    if args.is_empty() {
        return Err(ZystError::WrongNumberArgs);
    }

    Ok(Command {
        command_type: CommandType::SMEMBERS,
        args: CommandArgs::SingleKey(args[0].to_string()),
    })
}

pub fn build_srem_command(args: &[String]) -> Result<Command, ZystError> {
    if args.is_empty() {
        return Err(ZystError::WrongNumberArgs);
    }

    Ok(Command {
        command_type: CommandType::SREM,
        args: CommandArgs::KeyWithValues {
            key: args[0].to_string(),
            values: args.iter().skip(1).cloned().collect::<Vec<String>>(),
        },
    })
}
