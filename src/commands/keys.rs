use crate::errors::ZystError;
use crate::response::ZystResponse;
use crate::types::{Command, CommandArgs, Db, DbValue, Key};
use regex::Regex;

pub async fn get_key(db: &Db, command: Command) -> Result<ZystResponse, ZystError> {
    let key_name = match &command.args {
        CommandArgs::SingleKey(key) => &key.clone(),
        _ => return Err(ZystError::InvalidCommand),
    };

    let key = {
        let db_read = db.read().await;
        db_read.get(key_name).cloned() // Clone key to release lock
    };

    let key = match key {
        Some(DbValue::StringKey(k)) => k,
        None => return Ok(ZystResponse::Nil),
        Some(_) => return Err(ZystError::WrongType),
    };

    if let Some(value) = &key.data {
        let deleted = delete_expired_key(db, key.clone()).await; // No read lock at this point

        if !deleted {
            return Ok(ZystResponse::SimpleString(value.to_string()));
        }
    }

    Ok(ZystResponse::Nil)
}

pub async fn set_key(db: &Db, command: Command) -> Result<ZystResponse, ZystError> {
    let (key_name, value) = match command.args {
        CommandArgs::KeyWithValue { key, value } => (key, value),
        _ => return Err(ZystError::InvalidCommand),
    };

    let key = Key::new(key_name.clone(), Some(value.clone()), None);

    db.write()
        .await
        .insert(key.name.clone(), DbValue::StringKey(key));

    Ok(ZystResponse::Ok)
}

pub async fn delete_key(db: &Db, command: Command) -> Result<ZystResponse, ZystError> {
    let keys = match &command.args {
        CommandArgs::SingleKey(key) => vec![key.clone()],
        CommandArgs::MultipleKeys(keys) => keys.to_vec(),
        _ => return Err(ZystError::InvalidCommand),
    };

    let mut db_write = db.write().await;
    let mut deleted_count = 0;

    for key in keys {
        if db_write.swap_remove(&key).is_some() {
            deleted_count += 1;
        }
    }

    Ok(ZystResponse::Int(deleted_count))
}

// Increases the numeric value stored at the key by one.
// If the key does not exist, it is initialized to 0 before
// applying the operation. Returns an error if the key holds
// a non-numeric value or a string that cannot be interpreted
// as an integer.
pub async fn incr(db: &Db, command: Command) -> Result<ZystResponse, ZystError> {
    incr_decr(db, command, true).await
}

// Decrements the number stored at key by one.
// If the key does not exist, it is set to 0 before performing
// the operation. An error is returned if the key contains a value
// of the wrong type or contains a string that can not be
// represented as integer.
// This operation is limited to 64 bit signed integers.
pub async fn decr(db: &Db, command: Command) -> Result<ZystResponse, ZystError> {
    incr_decr(db, command, false).await
}

// Increases the number stored at the given key by the specified
// increment. If the key does not exist, it is initialized
// to 0 before applying the operation. Returns an error
// if the key holds a non-numeric value or a string that cannot
// be parsed as a 64-bit signed integer.
pub async fn incrby(db: &Db, command: Command) -> Result<ZystResponse, ZystError> {
    let (key_name, by_str) = match &command.args {
        CommandArgs::KeyWithValue { key, value } => (key.clone(), value.clone()),
        _ => return Err(ZystError::InvalidCommand),
    };

    let by = match by_str.parse::<i64>() {
        Ok(num) => num,
        Err(_) => return Err(ZystError::NotInt),
    };

    let mut db_write = db.write().await;

    let key = match db_write.get_mut(&key_name) {
        Some(DbValue::StringKey(existing_key)) => existing_key,
        None => {
            db_write.insert(
                key_name.clone(),
                DbValue::StringKey(Key {
                    name: key_name.clone(),
                    data: Some("0".to_string()),
                    ..Default::default()
                }),
            );
            match db_write.get_mut(&key_name) {
                Some(DbValue::StringKey(new_key)) => new_key,
                _ => return Err(ZystError::DatabaseError),
            }
        }
        Some(_) => return Err(ZystError::WrongType),
    };

    let num_str = key.data.as_deref().unwrap_or("0");

    let num = match num_str.parse::<i64>() {
        Ok(n) => n,
        Err(_) => return Err(ZystError::NotInt),
    };

    let new_value = num + by;
    key.data = Some(new_value.to_string());

    Ok(ZystResponse::Int(new_value))
}

async fn incr_decr(db: &Db, command: Command, inc: bool) -> Result<ZystResponse, ZystError> {
    let key_name = match command.args {
        CommandArgs::SingleKey(key) => key,
        _ => return Err(ZystError::InvalidCommand),
    };

    let mut db_write = db.write().await;

    let key = match db_write.get_mut(&key_name) {
        Some(DbValue::StringKey(key)) => key,
        None => {
            let key = Key::new(key_name.clone(), Some("0".to_string()), None);
            db_write.insert(key_name.clone(), DbValue::StringKey(key));
            match db_write.get_mut(&key_name) {
                Some(DbValue::StringKey(key)) => key,
                Some(DbValue::ListKey(_)) => return Err(ZystError::WrongType),
                _ => return Err(ZystError::DatabaseError),
            }
        }
        Some(_) => return Err(ZystError::WrongType),
    };

    let Ok(num) = key.data.as_deref().unwrap_or("0").parse::<i64>() else {
        return Err(ZystError::NotInt);
    };

    let new_value = if inc { num + 1 } else { num - 1 };

    key.data = Some(new_value.to_string());

    Ok(ZystResponse::Int(new_value))
}

/// Returns keys matching the Redis-style pattern
pub async fn get_keys(db: &Db, command: Command) -> Result<ZystResponse, ZystError> {
    let pattern = match &command.args {
        CommandArgs::SingleKey(key) => key,
        _ => return Err(ZystError::InvalidCommand),
    };

    // Convert Redis glob pattern to regex
    let regex_pattern = convert_redis_pattern_to_regex(pattern);
    let re = match Regex::new(&regex_pattern) {
        Ok(re) => re,
        Err(_) => return Err(ZystError::RegexError),
    };

    let mut results = vec![];

    let db_read = db.read().await;

    for key in db_read.keys() {
        if re.is_match(key) {
            results.push(key.clone());
        }
    }

    if results.is_empty() {
        return Ok(ZystResponse::EmptyArray);
    }

    Ok(ZystResponse::List(results))
}

pub async fn exists(db: &Db, command: Command) -> Result<ZystResponse, ZystError> {
    let keys = match &command.args {
        CommandArgs::SingleKey(key) => vec![key.to_string()],
        CommandArgs::MultipleKeys(keys) => keys.to_vec(),
        _ => return Err(ZystError::InvalidCommand),
    };

    let db_read = db.read().await;
    let nb_keys = keys.iter().filter(|key| db_read.contains_key(*key)).count() as i64;

    Ok(ZystResponse::Int(nb_keys))
}

pub async fn expire(db: &Db, command: Command) -> Result<ZystResponse, ZystError> {
    let (key_name, ttl) = match command.args {
        CommandArgs::KeyWithValue { key, value } => (key, value),
        _ => return Err(ZystError::InvalidCommand),
    };

    let ttl = ttl.parse::<i64>().map_err(|_| ZystError::TTL)?;

    let mut db_write = db.write().await;

    match db_write.get_mut(&key_name) {
        Some(DbValue::StringKey(key)) => {
            key.set_ttl(ttl);
            Ok(ZystResponse::Int(1))
        }
        Some(DbValue::ListKey(key)) => {
            key.set_ttl(ttl);
            Ok(ZystResponse::Int(1))
        }
        Some(DbValue::SetKey(key)) => {
            key.set_ttl(ttl);
            Ok(ZystResponse::Int(1))
        }
        Some(DbValue::HashKey(key)) => {
            key.set_ttl(ttl);
            Ok(ZystResponse::Int(1))
        }
        None => Ok(ZystResponse::Int(0)),
    }
}

pub async fn ttl(db: &Db, command: Command) -> Result<ZystResponse, ZystError> {
    let key_name = match command.args {
        CommandArgs::SingleKey(key) => key,
        _ => return Err(ZystError::InvalidCommand),
    };

    let db_read = db.read().await;

    let key = match db_read.get(&key_name) {
        Some(DbValue::StringKey(key)) => key,
        None => return Ok(ZystResponse::Int(-2)),
        Some(_) => return Err(ZystError::WrongType),
    };

    Ok(ZystResponse::Int(key.get_ttl()))
}

/// Converts Redis-style glob pattern into a valid regex pattern
// '*' becomes '.*'
// '?' becomes '.'
// '[' stays '[' (range starts)
// ']' stays ']' (range ends)
pub fn convert_redis_pattern_to_regex(pattern: &str) -> String {
    let mut regex_pattern = String::from("^");

    for c in pattern.chars() {
        match c {
            '*' => regex_pattern.push_str(".*"),
            '?' => regex_pattern.push('.'),
            '[' => regex_pattern.push('['),
            ']' => regex_pattern.push(']'),
            _ => regex_pattern.push_str(&regex::escape(&c.to_string())), // Escape other chars
        }
    }

    regex_pattern.push('$');
    regex_pattern
}

pub async fn delete_expired_key(db: &Db, key: Key) -> bool {
    let mut db_write = db.write().await;

    if key.is_expired() {
        db_write.swap_remove(&key.name);
        return true;
    }

    false
}
