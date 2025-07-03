use crate::errors::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use trust_dns_proto::rr::{Record, RrKey};

pub struct DnsCache {
    cache: Arc<RwLock<HashMap<RrKey, Vec<Record>>>>,
}

impl DnsCache {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get(&self, key: &RrKey) -> Option<Vec<Record>> {
        let cache = self.cache.read().await;
        cache.get(key).cloned()
    }

    pub async fn insert(&self, key: RrKey, records: Vec<Record>) {
        let mut cache = self.cache.write().await;
        cache.insert(key, records);
    }

    pub async fn clear(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }
}

impl Default for DnsCache {
    fn default() -> Self {
        Self::new()
    }
}