mod config;
mod monitoring;

use config::{get_config_path, load_or_create_config};
use monitoring::*;



#[tokio::main]
async fn main() {
    let config_path = get_config_path();
    let config = load_or_create_config(&config_path).expect("Failed to load config");
    println!("Config initiated: {:?}", config);

    monitor(&config).await;
}

