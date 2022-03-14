use crate::config::Config;
use rbatis::rbatis::Rbatis;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppContext {
    pub db: sea_orm::DatabaseConnection,
    pub rb: Arc<Rbatis>,
}

impl AppContext {
    pub async fn new(config: &crate::config::Config) -> Self {
        let db = sea_orm::Database::connect(config.postgres.db_url())
            .await
            .unwrap();

        let rb = Rbatis::new();
        rb.link(&config.postgres.db_url()).await.unwrap();

        Self {
            db,
            rb: Arc::new(rb),
        }
    }

    pub(crate) async fn new_for_mock_in_test() -> axum::extract::Extension<Self> {
        let config = std::fs::read_to_string("config.toml").unwrap();
        let config = toml::de::from_str::<Config>(&config).unwrap();
        axum::extract::Extension(Self::new(&config).await)
    }
}
