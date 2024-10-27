use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use models::user::User;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl CreateUserRequest {
    pub fn model(&self) -> User {
        User {
            id: 0,
            username: self.username.clone(),
            email: self.email.clone(),
            password_hash: Self::hash_password(&self.password),
            status: "active".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    fn hash_password(password: &str) -> String {
        hash(password, DEFAULT_COST).expect("Failed to hash password")
    }
}
