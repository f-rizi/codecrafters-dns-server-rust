use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use crate::dns::answer::Answer;
use crate::errors::DnsError;
use log::{info, warn};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

type CacheKey = (Vec<u8>, u16, u16);
type CacheValue = (Answer, Instant);

pub struct Cache {
    store: RwLock<HashMap<CacheKey, CacheValue>>,
    default_ttl: Duration,
}

impl Cache {
    pub fn new(default_ttl: Duration) -> Self {
        Self {
            store: RwLock::new(HashMap::new()),
            default_ttl,
        }
    }

    pub async fn get(&self, key: &CacheKey) -> Option<Answer> {
        let store = self.store.read().await;

        if let Some((answer, expiration)) = store.get(key) {
            if Instant::now() < *expiration {
                return Some(answer.clone());
            } else {
                warn!("Cache entry expired for key: {:?}", key);
                //store.remove(key);
            }
        }

        info!("Cache miss for key: {:?}", key);
        None
    }

    pub async fn insert(&self, key: CacheKey, answer: Answer, ttl: Option<Duration>) {
        let mut store = self.store.write().await;
        let expiration = Instant::now() + ttl.unwrap_or(self.default_ttl);

        store.insert(key, (answer, expiration));
    }
}
