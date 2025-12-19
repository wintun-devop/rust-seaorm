mod config;




#[tokio::main]
async fn main() {
    let db_url = config::config().db_url;
    println!("Hello, world!");
    println!("database url is:{}",db_url);
}
