use crate::errors::ZystError;
use crate::response::ZystResponse;
use crate::types::{Command, CommandArgs, Db, DbValue, KeyList, ListPushType, PopType};

async fn push_to_list(
    db: &Db,
    command: Command,
    push_type: ListPushType,
) -> Result<ZystResponse, ZystError> {
    let (key_name, values) = match command.args {
        CommandArgs::KeyWithValues { key, values } => (key, values),
        _ => return Err(ZystError::InvalidCommand),
    };

    let mut new_values = values;

    let mut db_write = db.write().await;

    match db_write.get_mut(&key_name) {
        Some(DbValue::ListKey(existing_list)) => {
            match push_type {
                ListPushType::LPUSH => {
                    for value in new_values.into_iter().rev() {
                        existing_list.data.push_front(value);
                    }
                }
                ListPushType::RPUSH => {
                    existing_list.data.extend(new_values);
                }
            }
            let nb = existing_list.data.len() as i64;
            Ok(ZystResponse::Int(nb))
        }
        None => {
            if let ListPushType::LPUSH = push_type {
                new_values.reverse();
            }
            db_write.insert(
                key_name.clone(),
                DbValue::ListKey(KeyList {
                    name: key_name,
                    data: new_values.clone().into(),
                    ..Default::default()
                }),
            );
            let nb = new_values.len() as i64;
            Ok(ZystResponse::Int(nb))
        }
        Some(_) => Err(ZystError::WrongType),
    }
}

pub async fn lpush(db: &Db, command: Command) -> Result<ZystResponse, ZystError> {
    push_to_list(db, command, ListPushType::LPUSH).await
}

pub async fn rpush(db: &Db, command: Command) -> Result<ZystResponse, ZystError> {
    push_to_list(db, command, ListPushType::RPUSH).await
}

pub async fn lrange(db: &Db, command: Command) -> Result<ZystResponse, ZystError> {
    let (key_name, values) = match command.args {
        CommandArgs::KeyWithValues { key, values } => (key, values),
        _ => return Err(ZystError::InvalidCommand),
    };

    let min: isize = match values[0].parse::<isize>() {
        Ok(val) => val,
        Err(_) => return Err(ZystError::NotIntOrOutOfRange),
    };

    let max: isize = match values[1].parse::<isize>() {
        Ok(val) => val,
        Err(_) => return Err(ZystError::NotIntOrOutOfRange),
    };

    let db_read = db.read().await;

    let key = match db_read.get(&key_name) {
        Some(DbValue::ListKey(key)) => key,
        Some(DbValue::StringKey(_)) => return Err(ZystError::WrongType),
        _ => return Ok(ZystResponse::EmptyArray),
    };

    let len = key.data.len();

    let min = if min >= 0 {
        min as usize
    } else {
        (len as isize + min).max(0) as usize
    };

    let max = if max >= 0 {
        (max + 1).min(len as isize) as usize
    } else {
        (len as isize + max + 1).max(0).min(len as isize) as usize
    };

    if min >= max || min >= len {
        return Ok(ZystResponse::EmptyArray);
    }

    let results: Vec<String> = key.data.range(min..max).cloned().collect();

    if results.is_empty() {
        return Ok(ZystResponse::EmptyArray);
    }

    Ok(ZystResponse::List(results.to_vec()))
}

pub async fn lpop(db: &Db, command: Command) -> Result<ZystResponse, ZystError> {
    pop_list(db, command, PopType::LPOP).await
}

pub async fn rpop(db: &Db, command: Command) -> Result<ZystResponse, ZystError> {
    pop_list(db, command, PopType::RPOP).await
}

async fn pop_list(
    db: &Db,
    command: Command,
    pop_type: PopType,
) -> Result<ZystResponse, ZystError> {
    let (key_name, value) = match &command.args {
        CommandArgs::SingleKey(key) => (key.clone(), None),
        CommandArgs::KeyWithValue { key, value } => (key.clone(), Some(value.clone())),
        _ => return Err(ZystError::InvalidCommand),
    };

    let mut db_write = db.write().await;

    let key_db = match db_write.get_mut(&key_name) {
        Some(DbValue::ListKey(key)) => key,
        Some(DbValue::StringKey(_)) => return Err(ZystError::WrongType),
        _ => return Ok(ZystResponse::EmptyArray),
    };

    let nb = value
        .as_deref()
        .map_or(1, |v| v.parse::<usize>().unwrap_or(1));

    let len = key_db.data.len();

    let (start, end) = match pop_type {
        PopType::LPOP => (0, nb),
        PopType::RPOP => (len.saturating_sub(nb), len),
    };

    let mut removed: Vec<String> = key_db
        .data
        .drain(start..end.min(key_db.data.len()))
        .collect();

    if removed.is_empty() {
        return Ok(ZystResponse::Nil);
    }

    if let PopType::RPOP = pop_type {
        removed.reverse();
    }

    // If LPOP is passed without arguments, return the first element
    if nb == 1 {
        return Ok(ZystResponse::SimpleString(removed[0].clone()));
    }

    Ok(ZystResponse::List(removed))
}
