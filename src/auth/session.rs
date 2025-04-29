use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct SessionInfo {
    pub username: String,
    pub created_at: Instant,
}

#[derive(Clone)]
pub struct SessionManager {
    sessions: Arc<Mutex<HashMap<String, SessionInfo>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn insert(&self, token: String, username: String) {
        let mut sessions = self.sessions.lock().await;
        sessions.insert(token, SessionInfo {
            username,
            created_at: Instant::now(),
        });
    }

    pub async fn validate(&self, token: &str) -> bool {
        let sessions = self.sessions.lock().await;
        sessions.get(token).map_or(false, |session| {
            session.created_at.elapsed() < Duration::from_secs(1800) // 30분 만료
        })
    }

    pub async fn remove(&self, token: &str) {
        let mut sessions = self.sessions.lock().await;
        sessions.remove(token);
    }

    pub async fn cleanup_expired(&self) {
        let mut sessions = self.sessions.lock().await;
        sessions.retain(|_, session| session.created_at.elapsed() < Duration::from_secs(1800));
    }
}
