use sqlx::PgPool;
use multitool_hg::rediska::client::Rediska;
use models::user::User;
use repository::user::UserRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthState {
    pub database_pool: Arc<PgPool>,
    pub redis_pool: Arc<Rediska>,
}

impl AuthState {
    pub fn new(database_pool: PgPool, redis_pool: Rediska) -> Self {
        Self {
            database_pool: Arc::new(database_pool),
            redis_pool: Arc::new(redis_pool),
        }
    }

    pub async fn create_user(&self, user: User) -> anyhow::Result<User> {
        let repo = UserRepository::new(&self.database_pool);
        repo.create(&user).await
    }

    pub async fn cache_user_data(&self, user_id: i32, data: String) -> anyhow::Result<()> {
        self.redis_pool.set(&format!("user:{}", user_id), &data, None).await
    }
}
