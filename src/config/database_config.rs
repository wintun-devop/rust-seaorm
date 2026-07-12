use dotenvy::dotenv;
use std::env;

pub struct DatabaseConfig {
    pub db_url: String,
}

pub fn database_config() -> DatabaseConfig {
    dotenv().ok();
    DatabaseConfig {
        db_url: env::var("DATABASE_URL").expect("DATABASE_URL not set!"),
    }
}
