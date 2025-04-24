use crate::auth::auth_packet::{AuthRequest, AuthResponse};
use crate::auth::token;
use std::collections::HashMap;

// 임시 유저 DB
pub struct AuthService {
    users: HashMap<String, String>, // username -> password
}

impl AuthService {
    pub fn new() -> Self {
        let mut users = HashMap::new();
        users.insert("player1".to_string(), "1234".to_string());
        Self { users }
    }

    pub fn authenticate(&self, req: AuthRequest) -> AuthResponse {
        match self.users.get(&req.username) {
            Some(pw) if pw == &req.password => {
                let token = token::generate_token(&req.username);
                AuthResponse {
                    success: true,
                    token: Some(token),
                    message: None,
                }
            }
            _ => AuthResponse {
                success: false,
                token: None,
                message: Some("Invalid credentials".to_string()),
            },
        }
    }
}
