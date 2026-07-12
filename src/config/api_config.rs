use dotenvy::dotenv;
use std::env;

pub struct ApiConfig {
    pub secret_key: String,
}

pub fn api_config() -> ApiConfig {
    dotenv().ok();
    ApiConfig {
        secret_key: env::var("SECRET_KEY").expect("SECRET_KEY not set!"),
    }
}
