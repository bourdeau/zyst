use crate::errors::ZystError;
use crate::response::ZystResponse;
use crate::types::Command;
use crate::types::CommandArgs;
use crate::types::Db;
use crate::types::DbValue;
use crate::types::KeyHash;

pub async fn hset(db: &Db, command: Command) -> Result<ZystResponse, ZystError> {
    let (key_name, key_values) = match &command.args {
        CommandArgs::HashFields { key, fields } => (key.to_string(), fields),
        _ => return Err(ZystError::InvalidCommand),
    };

    let key = {
        let db_read = db.read().await;
        db_read.get(&key_name).cloned()
    };

    let mut db_write = db.write().await;

    let nb = match key {
        Some(DbValue::HashKey(mut k)) => {
            let before_len = k.data.len();
            k.data.extend(key_values.clone());
            let after_len = k.data.len();
            db_write.insert(key_name, DbValue::HashKey(k));
            after_len - before_len
        }
        None => {
            db_write.insert(
                key_name.clone(),
                DbValue::HashKey(KeyHash {
                    name: key_name,
                    data: key_values.clone(),
                    ..Default::default()
                }),
            );
            key_values.len()
        }
        Some(_) => return Err(ZystError::WrongType),
    };

    Ok(ZystResponse::Int(nb as i64))
}

pub async fn hget(db: &Db, command: Command) -> Result<ZystResponse, ZystError> {
    let (hash_name, field_name) = match &command.args {
        CommandArgs::KeyWithValue { key, value } => (key, value),
        _ => return Err(ZystError::InvalidCommand),
    };

    let db_read = db.read().await;

    match db_read.get(hash_name) {
        Some(DbValue::HashKey(hash)) => match hash.data.get(field_name) {
            Some(value) => Ok(ZystResponse::SimpleString(value.to_string())),
            None => Ok(ZystResponse::Nil),
        },
        None => Ok(ZystResponse::Nil),
        Some(_) => Err(ZystError::WrongType),
    }
}

pub async fn hgetall(db: &Db, command: Command) -> Result<ZystResponse, ZystError> {
    let key_name = match &command.args {
        CommandArgs::SingleKey(key_name) => key_name,
        _ => return Err(ZystError::InvalidCommand),
    };

    let db_read = db.read().await;

    let results = match db_read.get(key_name) {
        Some(DbValue::HashKey(key)) => key
            .data
            .iter()
            .flat_map(|(k, v)| vec![k.clone(), v.clone()])
            .collect::<Vec<String>>(),
        Some(_) => return Err(ZystError::WrongType),
        None => return Ok(ZystResponse::EmptyArray),
    };

    Ok(ZystResponse::List(results))
}

pub async fn hdel(db: &Db, command: Command) -> Result<ZystResponse, ZystError> {
    let (key_name, fields) = match &command.args {
        CommandArgs::KeyWithValues { key, values } => (key.clone(), values.clone()),
        _ => return Err(ZystError::InvalidCommand),
    };

    let mut db_write = db.write().await;

    match db_write.get_mut(&key_name) {
        Some(DbValue::HashKey(hash)) => {
            let mut deleted_count = 0;
            for field in fields {
                if hash.data.swap_remove(&field).is_some() {
                    deleted_count += 1;
                }
            }

            if hash.data.is_empty() {
                db_write.swap_remove(&key_name);
            }
            Ok(ZystResponse::Int(deleted_count))
        }
        Some(_) => Err(ZystError::WrongType),
        None => Ok(ZystResponse::Int(0)),
    }
}
