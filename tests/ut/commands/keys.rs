#[cfg(test)]
mod tests {
    use indexmap::IndexMap;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use zyst::commands::keys::*;
    use zyst::types::*;

    async fn setup_db() -> Db {
        Arc::new(RwLock::new(IndexMap::new()))
    }

    #[tokio::test]
    async fn test_set_key() {
        let db = setup_db().await;
        let command = Command {
            command_type: CommandType::SET,
            args: CommandArgs::KeyWithValue {
                key: "my_key".to_string(),
                value: "value".to_string(),
            },
        };

        let result = set_key(&db, command).await.unwrap().to_string();
        assert_eq!(result, "+OK\r\n");

        let db_read = db.read().await;
        assert!(db_read.contains_key("my_key"));
    }

    #[tokio::test]
    async fn test_delete_key() {
        let db = setup_db().await;
        let key_name = "key_to_delete".to_string();

        {
            let mut db_write = db.write().await;
            db_write.insert(
                key_name.clone(),
                DbValue::StringKey(Key {
                    name: key_name.clone(),
                    data: Some("value".to_string()),
                    expires_at: None,
                }),
            );
        }

        let command = Command {
            command_type: CommandType::DEL,
            args: CommandArgs::SingleKey(key_name.clone()),
        };

        let result = delete_key(&db, command).await.unwrap().to_string();
        assert_eq!(result, "+(integer) 1\r\n");

        let db_read = db.read().await;
        assert!(!db_read.contains_key(&key_name));
    }

    #[tokio::test]
    async fn test_incr_new_key() {
        let db = setup_db().await;
        let command = Command {
            command_type: CommandType::INCR,
            args: CommandArgs::SingleKey("counter".to_string()),
        };

        let result = incr(&db, command).await.unwrap().to_string();
        assert_eq!(result, "+(integer) 1\r\n");
    }

    #[tokio::test]
    async fn test_incr_existing_key() {
        let db = setup_db().await;
        let key_name = "counter".to_string();

        {
            let mut db_write = db.write().await;
            db_write.insert(
                key_name.clone(),
                DbValue::StringKey(Key {
                    name: key_name.clone(),
                    data: Some("5".to_string()),
                    expires_at: None,
                }),
            );
        }

        let command = Command {
            command_type: CommandType::INCR,
            args: CommandArgs::SingleKey(key_name),
        };

        let result = incr(&db, command).await.unwrap().to_string();
        assert_eq!(result, "+(integer) 6\r\n");
    }

    #[tokio::test]
    async fn test_decr_new_key() {
        let db = setup_db().await;
        let command = Command {
            command_type: CommandType::DECR,
            args: CommandArgs::SingleKey("counter".to_string()),
        };

        let result = decr(&db, command).await.unwrap().to_string();
        assert_eq!(result, "+(integer) -1\r\n");
    }

    #[tokio::test]
    async fn test_incrby() {
        let db = setup_db().await;
        let key_name = "counter".to_string();

        {
            let mut db_write = db.write().await;
            db_write.insert(
                key_name.clone(),
                DbValue::StringKey(Key {
                    name: key_name.clone(),
                    data: Some("10".to_string()),
                    expires_at: None,
                }),
            );
        }

        let command = Command {
            command_type: CommandType::INCRBY,
            args: CommandArgs::KeyWithValue {
                key: key_name,
                value: "5".to_string(),
            },
        };

        let result = incrby(&db, command).await.unwrap().to_string();
        assert_eq!(result, "+(integer) 15\r\n");
    }

    #[tokio::test]
    async fn test_get_keys() {
        let db = setup_db().await;

        {
            let mut db_write = db.write().await;
            db_write.insert(
                "foo".to_string(),
                DbValue::StringKey(Key {
                    name: "foo".to_string(),
                    data: Some("bar".to_string()),
                    expires_at: None,
                }),
            );
            db_write.insert(
                "foobar".to_string(),
                DbValue::StringKey(Key {
                    name: "foobar".to_string(),
                    data: Some("baz".to_string()),
                    expires_at: None,
                }),
            );
        }

        let command = Command {
            command_type: CommandType::KEYS,
            args: CommandArgs::SingleKey("foo*".to_string()),
        };

        let result = get_keys(&db, command).await.unwrap().to_string();
        assert_eq!(result, "*2\r\n$3\r\nfoo\r\n$6\r\nfoobar\r\n");
    }

    #[tokio::test]
    async fn test_exists() {
        let db = setup_db().await;

        {
            let mut db_write = db.write().await;
            db_write.insert(
                "key1".to_string(),
                DbValue::StringKey(Key {
                    name: "key1".to_string(),
                    data: Some("val1".to_string()),
                    expires_at: None,
                }),
            );
        }

        let command = Command {
            command_type: CommandType::EXISTS,
            args: CommandArgs::MultipleKeys(vec!["key1".to_string(), "key2".to_string()]),
        };

        let result = exists(&db, command).await.unwrap().to_string();
        assert_eq!(result, "+(integer) 1\r\n");
    }
}
