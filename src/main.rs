mod config;
mod models;

// import the database_config and api_config modules
use crate::config::api_config::api_config;
use crate::config::database_config::connect_db;
use crate::config::database_config::database_config;

#[tokio::main]
async fn main() {
    let db_url = database_config().db_url;
    let api_key = api_config().secret_key;
    let db = connect_db().await.expect("Database connection failed");
    println!("Hello, world!");
    println!("database url is:{}", db_url);
    println!("API key is:{}", api_key);
    println!("{:?}", db);
}
