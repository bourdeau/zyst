use crate::types::KeyBase;
use std::time::{SystemTime, UNIX_EPOCH};

impl<T> KeyBase<T> {
    pub fn new(name: String, data: T, expires_at: Option<i64>) -> Self {
        KeyBase {
            name,
            data,
            expires_at,
        }
    }

    fn get_current_timestamp() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as i64
    }

    pub fn get_ttl(&self) -> i64 {
        self.expires_at
            .map_or(-1, |expires_at| expires_at - Self::get_current_timestamp())
    }

    pub fn set_ttl(&mut self, ttl: i64) {
        self.expires_at = Some(Self::get_current_timestamp() + ttl);
    }

    // A key without ttl returns -1 and is not expired
    pub fn is_expired(&self) -> bool {
        match self.expires_at {
            Some(expires_at) => expires_at <= Self::get_current_timestamp(),
            None => false,
        }
    }
}
