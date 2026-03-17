use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::error::AppError;

pub async fn init_pool(database_url: &str) -> Result<PgPool, AppError> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .map_err(|e| AppError::Database(format!("failed to connect to db: {e}")))
}
