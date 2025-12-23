use redis::AsyncCommands;
use sha2::{Digest, Sha256};

#[derive(Clone)]
pub struct RedisCache {
    client: redis::Client,
}

impl RedisCache {
    pub fn new(redis_url: &str) -> anyhow::Result<Self> {
        Ok(Self { client: redis::Client::open(redis_url)? })
    }

    fn key_for(query: &str, variables_json: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(query.as_bytes());
        hasher.update(b"|");
        hasher.update(variables_json.as_bytes());
        format!("gql:{}", hex::encode(hasher.finalize()))
    }

    pub async fn get(&self, query: &str, variables_json: &str) -> anyhow::Result<Option<String>> {
        let key = Self::key_for(query, variables_json);
        let mut conn = self.client.get_multiplexed_tokio_connection().await?;
        let val: Option<String> = conn.get(key).await?;
        Ok(val)
    }

    pub async fn set_ex(&self, query: &str, variables_json: &str, value: &str, ttl_seconds: usize) -> anyhow::Result<()> {
        let key = Self::key_for(query, variables_json);
        let mut conn = self.client.get_multiplexed_tokio_connection().await?;
        let _: () = conn.set_ex(key, value, ttl_seconds).await?;
        Ok(())
    }
}
