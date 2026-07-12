mod config;




// import the database_config and api_config modules
use crate::config::database_config::database_config;
use crate::config::api_config::api_config;





#[tokio::main]
async fn main() {
    let db_url = database_config().db_url;
    let api_key = api_config().secret_key;
    println!("Hello, world!");
    println!("database url is:{}",db_url);
    println!("API key is:{}",api_key);
}
