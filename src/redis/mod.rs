use std::time::Duration;
use bb8::{Pool, PooledConnection};
use bb8_redis::RedisConnectionManager;
use redis::{AsyncCommands, RedisResult};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, warn};

use crate::config::RedisConfig;
use crate::errors::{AppError, Result};

pub type RedisPool = Pool<RedisConnectionManager>;
pub type RedisConnection = PooledConnection<'static, RedisConnectionManager>;

#[derive(Clone)]
pub struct RedisClient {
    pool: RedisPool,
    key_prefix: String,
}

impl RedisClient {
    pub async fn new(config: &RedisConfig) -> Result<Self> {
        let manager = RedisConnectionManager::new(config.uri.as_str())
            .map_err(|e| AppError::Internal(format!("Failed to create Redis manager: {}", e)))?;

        let pool = Pool::builder()
            .max_size(config.max_connections)
            .connection_timeout(Duration::from_secs(5))
            .idle_timeout(Some(Duration::from_secs(300)))
            .max_lifetime(Some(Duration::from_secs(3600)))
            .build(manager)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to create Redis pool: {}", e)))?;

        info!("Redis connection pool created with {} max connections", config.max_connections);

        Ok(Self {
            pool,
            key_prefix: config.key_prefix.clone(),
        })
    }

    pub async fn get_connection(&self) -> Result<PooledConnection<'_, RedisConnectionManager>> {
        self.pool
            .get()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to get Redis connection: {}", e)))
    }

    fn prefixed_key(&self, key: &str) -> String {
        format!("{}{}", self.key_prefix, key)
    }

    pub async fn set<T>(&self, key: &str, value: &T, ttl: Option<u64>) -> Result<()>
    where
        T: Serialize,
    {
        let mut conn = self.get_connection().await?;
        let prefixed_key = self.prefixed_key(key);
        let serialized = serde_json::to_string(value)
            .map_err(|e| AppError::Serialization(e))?;

        if let Some(ttl) = ttl {
            conn.set_ex(prefixed_key, serialized, ttl)
                .await
                .map_err(|e| AppError::Internal(format!("Redis SET_EX failed: {}", e)))?;
        } else {
            conn.set(prefixed_key, serialized)
                .await
                .map_err(|e| AppError::Internal(format!("Redis SET failed: {}", e)))?;
        }

        debug!("Set key: {} with TTL: {:?}", key, ttl);
        Ok(())
    }

    pub async fn get<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut conn = self.get_connection().await?;
        let prefixed_key = self.prefixed_key(key);

        let value: Option<String> = conn
            .get(prefixed_key)
            .await
            .map_err(|e| AppError::Internal(format!("Redis GET failed: {}", e)))?;

        match value {
            Some(serialized) => {
                let deserialized = serde_json::from_str(&serialized)
                    .map_err(|e| AppError::Serialization(e))?;
                debug!("Retrieved key: {}", key);
                Ok(Some(deserialized))
            }
            None => {
                debug!("Key not found: {}", key);
                Ok(None)
            }
        }
    }

    pub async fn delete(&self, key: &str) -> Result<bool> {
        let mut conn = self.get_connection().await?;
        let prefixed_key = self.prefixed_key(key);

        let deleted: i32 = conn
            .del(prefixed_key)
            .await
            .map_err(|e| AppError::Internal(format!("Redis DEL failed: {}", e)))?;

        debug!("Deleted key: {} (existed: {})", key, deleted > 0);
        Ok(deleted > 0)
    }

    pub async fn exists(&self, key: &str) -> Result<bool> {
        let mut conn = self.get_connection().await?;
        let prefixed_key = self.prefixed_key(key);

        let exists: bool = conn
            .exists(prefixed_key)
            .await
            .map_err(|e| AppError::Internal(format!("Redis EXISTS failed: {}", e)))?;

        Ok(exists)
    }

    pub async fn increment(&self, key: &str, ttl: Option<u64>) -> Result<i64> {
        let mut conn = self.get_connection().await?;
        let prefixed_key = self.prefixed_key(key);

        let value: i64 = conn
            .incr(prefixed_key.clone(), 1)
            .await
            .map_err(|e| AppError::Internal(format!("Redis INCR failed: {}", e)))?;

        if let Some(ttl) = ttl {
            conn.expire(prefixed_key, ttl as usize)
                .await
                .map_err(|e| AppError::Internal(format!("Redis EXPIRE failed: {}", e)))?;
        }

        debug!("Incremented key: {} to value: {}", key, value);
        Ok(value)
    }

    pub async fn set_add(&self, key: &str, member: &str, ttl: Option<u64>) -> Result<bool> {
        let mut conn = self.get_connection().await?;
        let prefixed_key = self.prefixed_key(key);

        let added: i32 = conn
            .sadd(prefixed_key.clone(), member)
            .await
            .map_err(|e| AppError::Internal(format!("Redis SADD failed: {}", e)))?;

        if let Some(ttl) = ttl {
            conn.expire(prefixed_key, ttl as usize)
                .await
                .map_err(|e| AppError::Internal(format!("Redis EXPIRE failed: {}", e)))?;
        }

        debug!("Added member {} to set {}", member, key);
        Ok(added > 0)
    }

    pub async fn set_is_member(&self, key: &str, member: &str) -> Result<bool> {
        let mut conn = self.get_connection().await?;
        let prefixed_key = self.prefixed_key(key);

        let is_member: bool = conn
            .sismember(prefixed_key, member)
            .await
            .map_err(|e| AppError::Internal(format!("Redis SISMEMBER failed: {}", e)))?;

        Ok(is_member)
    }

    pub async fn hash_set(&self, key: &str, field: &str, value: &str, ttl: Option<u64>) -> Result<()> {
        let mut conn = self.get_connection().await?;
        let prefixed_key = self.prefixed_key(key);

        conn.hset(prefixed_key.clone(), field, value)
            .await
            .map_err(|e| AppError::Internal(format!("Redis HSET failed: {}", e)))?;

        if let Some(ttl) = ttl {
            conn.expire(prefixed_key, ttl as usize)
                .await
                .map_err(|e| AppError::Internal(format!("Redis EXPIRE failed: {}", e)))?;
        }

        debug!("Set hash field {}:{} in key {}", field, value, key);
        Ok(())
    }

    pub async fn hash_get(&self, key: &str, field: &str) -> Result<Option<String>> {
        let mut conn = self.get_connection().await?;
        let prefixed_key = self.prefixed_key(key);

        let value: Option<String> = conn
            .hget(prefixed_key, field)
            .await
            .map_err(|e| AppError::Internal(format!("Redis HGET failed: {}", e)))?;

        Ok(value)
    }

    pub async fn list_push(&self, key: &str, value: &str, ttl: Option<u64>) -> Result<i64> {
        let mut conn = self.get_connection().await?;
        let prefixed_key = self.prefixed_key(key);

        let length: i64 = conn
            .lpush(prefixed_key.clone(), value)
            .await
            .map_err(|e| AppError::Internal(format!("Redis LPUSH failed: {}", e)))?;

        if let Some(ttl) = ttl {
            conn.expire(prefixed_key, ttl as usize)
                .await
                .map_err(|e| AppError::Internal(format!("Redis EXPIRE failed: {}", e)))?;
        }

        debug!("Pushed value to list {}, new length: {}", key, length);
        Ok(length)
    }

    pub async fn list_range(&self, key: &str, start: isize, stop: isize) -> Result<Vec<String>> {
        let mut conn = self.get_connection().await?;
        let prefixed_key = self.prefixed_key(key);

        let values: Vec<String> = conn
            .lrange(prefixed_key, start, stop)
            .await
            .map_err(|e| AppError::Internal(format!("Redis LRANGE failed: {}", e)))?;

        Ok(values)
    }

    pub async fn publish(&self, channel: &str, message: &str) -> Result<i32> {
        let mut conn = self.get_connection().await?;
        let prefixed_channel = self.prefixed_key(channel);

        let subscribers: i32 = conn
            .publish(prefixed_channel, message)
            .await
            .map_err(|e| AppError::Internal(format!("Redis PUBLISH failed: {}", e)))?;

        debug!("Published message to channel {} (subscribers: {})", channel, subscribers);
        Ok(subscribers)
    }

    pub async fn get_pool_status(&self) -> PoolStatus {
        let state = self.pool.state();
        PoolStatus {
            connections: state.connections,
            idle_connections: state.idle_connections,
            max_size: self.pool.max_size(),
        }
    }

    // Rate limiting functionality
    pub async fn rate_limit_check(&self, key: &str, limit: u32, window_seconds: u64) -> Result<RateLimitResult> {
        let mut conn = self.get_connection().await?;
        let prefixed_key = self.prefixed_key(&format!("rate_limit:{}", key));
        
        let current_count: i64 = conn
            .incr(prefixed_key.clone(), 1)
            .await
            .map_err(|e| AppError::Internal(format!("Redis rate limit INCR failed: {}", e)))?;

        if current_count == 1 {
            conn.expire(prefixed_key, window_seconds as usize)
                .await
                .map_err(|e| AppError::Internal(format!("Redis rate limit EXPIRE failed: {}", e)))?;
        }

        let allowed = current_count <= limit as i64;
        let remaining = if allowed { (limit as i64 - current_count).max(0) } else { 0 };

        Ok(RateLimitResult {
            allowed,
            count: current_count as u32,
            limit,
            remaining: remaining as u32,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolStatus {
    pub connections: u32,
    pub idle_connections: u32,
    pub max_size: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RateLimitResult {
    pub allowed: bool,
    pub count: u32,
    pub limit: u32,
    pub remaining: u32,
}

// DNS-specific caching operations
impl RedisClient {
    pub async fn cache_dns_response(&self, domain: &str, record_type: &str, response: &str, ttl: u64) -> Result<()> {
        let key = format!("dns_cache:{}:{}", domain, record_type);
        self.set(&key, response, Some(ttl)).await
    }

    pub async fn get_cached_dns_response(&self, domain: &str, record_type: &str) -> Result<Option<String>> {
        let key = format!("dns_cache:{}:{}", domain, record_type);
        self.get(&key).await
    }

    pub async fn cache_threat_result(&self, domain: &str, is_threat: bool, ttl: u64) -> Result<()> {
        let key = format!("threat_cache:{}", domain);
        self.set(&key, &is_threat, Some(ttl)).await
    }

    pub async fn get_cached_threat_result(&self, domain: &str) -> Result<Option<bool>> {
        let key = format!("threat_cache:{}", domain);
        self.get(&key).await
    }

    pub async fn add_to_blocklist(&self, domain: &str) -> Result<()> {
        self.set_add("blocklist", domain, None).await?;
        Ok(())
    }

    pub async fn is_in_blocklist(&self, domain: &str) -> Result<bool> {
        self.set_is_member("blocklist", domain).await
    }

    pub async fn log_query_stats(&self, metric: &str, value: i64) -> Result<()> {
        let timestamp = chrono::Utc::now().timestamp();
        let key = format!("stats:{}:{}", metric, timestamp / 300); // 5-minute buckets
        
        self.increment(&key, Some(3600)).await?; // Keep for 1 hour
        Ok(())
    }
}