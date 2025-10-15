mod config;

use config::{get_config_path, load_or_create_config};

use reqwest;
use tokio::time::{sleep, Duration};
use std::collections::HashMap;


#[tokio::main]
async fn main() {
    let config_path = get_config_path();
    let config = load_or_create_config(&config_path).expect("Failed to load config");
    println!("Config initiated: {:?}", config);

    let mut status_map: HashMap<String, bool> = HashMap::new();

    loop {
        for url in &config.servers {
            let is_up = check_server(url).await;
            let previous_status = status_map.get(url).copied().unwrap_or(false);

            if is_up && !previous_status {
                println!("Server {} is up", url);
            }
            status_map.insert(url.to_string(), is_up);
        }
        let check_interval_seconds = config.check_interval_seconds;
        println!("Sleeping for {:?} seconds", check_interval_seconds);
        sleep(Duration::from_secs(check_interval_seconds)).await;
    }
}

async fn check_server(url: &str) -> bool {
    match reqwest::get(url).await {
        Ok(resp) => resp.status().is_success(),
        Err(_) => false,
    }
}