use dotenvy::dotenv;
use std::env;

use sea_orm::{Database, DatabaseConnection, DbErr};

pub struct DatabaseConfig {
    pub db_url: String,
}

pub fn database_config() -> DatabaseConfig {
    dotenv().ok();
    DatabaseConfig {
        db_url: env::var("DATABASE_URL").expect("DATABASE_URL not set!"),
    }
}

pub async fn connect_db() -> Result<DatabaseConnection, DbErr> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    Database::connect(database_url).await
}
