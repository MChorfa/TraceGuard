use redis::{Client, Commands, RedisResult};
use serde::{Deserialize, Serialize};

pub struct RedisCache {
    client: Client,
}

impl RedisCache {
    pub fn new(redis_url: &str) -> RedisResult<Self> {
        let client = Client::open(redis_url)?;
        Ok(Self { client })
    }

    pub fn set<T: Serialize>(&self, key: &str, value: &T, expiration: usize) -> RedisResult<()> {
        let mut con = self.client.get_connection()?;
        let serialized = serde_json::to_string(value).unwrap();
        con.set_ex(key, serialized, expiration)
    }

    pub fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> RedisResult<Option<T>> {
        let mut con = self.client.get_connection()?;
        let result: Option<String> = con.get(key)?;
        Ok(result.map(|s| serde_json::from_str(&s).unwrap()))
    }
}