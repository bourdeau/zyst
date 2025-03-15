use crate::errors::ZystError;
use crate::response::ZystResponse;
use crate::types::{Command, CommandArgs, Db, DbValue, KeySet};
use std::collections::HashSet;

pub async fn sadd(db: &Db, command: Command) -> Result<ZystResponse, ZystError> {
    let (set_name, values) = match command.args {
        CommandArgs::KeyWithValues { key, values } => (key, values),
        _ => return Err(ZystError::InvalidCommand),
    };

    let mut db_write = db.write().await;

    match db_write.get_mut(&set_name) {
        Some(DbValue::SetKey(db_set)) => {
            db_set.data.extend(values);
            let nb = db_set.data.len() as i64;
            Ok(ZystResponse::Int(nb))
        }
        None => {
            let new_set = DbValue::SetKey(KeySet {
                name: set_name.clone(),
                data: HashSet::from_iter(values.clone()),
                ..Default::default()
            });
            db_write.insert(set_name, new_set);
            let nb = values.len() as i64;
            Ok(ZystResponse::Int(nb))
        }
        Some(_) => Err(ZystError::WrongType),
    }
}

pub async fn smembers(db: &Db, command: Command) -> Result<ZystResponse, ZystError> {
    let key_name = match &command.args {
        CommandArgs::SingleKey(key_name) => key_name,
        _ => return Err(ZystError::InvalidCommand),
    };

    let db_read = db.read().await;

    let results = match db_read.get(key_name) {
        Some(DbValue::SetKey(key)) => key.data.iter().cloned().collect::<Vec<String>>(),
        None => return Ok(ZystResponse::EmptyArray),
        Some(_) => return Err(ZystError::WrongType),
    };

    Ok(ZystResponse::List(results))
}

pub async fn srem(db: &Db, command: Command) -> Result<ZystResponse, ZystError> {
    let (set_name, members) = match &command.args {
        CommandArgs::KeyWithValues { key, values } => (key.clone(), values.clone()),
        _ => return Err(ZystError::InvalidCommand),
    };

    let mut db_write = db.write().await;

    match db_write.get_mut(&set_name) {
        Some(DbValue::SetKey(key)) => {
            let mut deleted_count = 0;
            for member in members {
                if key.data.remove(&member) {
                    deleted_count += 1;
                }
            }

            if key.data.is_empty() {
                db_write.swap_remove(&set_name);
            }
            Ok(ZystResponse::Int(deleted_count))
        }
        Some(_) => Err(ZystError::WrongType),
        None => Ok(ZystResponse::Int(0)),
    }
}
