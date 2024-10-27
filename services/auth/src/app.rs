use sqlx::PgPool;
use multitool_hg::rediska::client::Rediska;
use models::user::User;
use repository::user::UserRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub database_pool: Arc<PgPool>,
    pub redis_pool: Arc<Rediska>,
}

impl AppState {
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
}
