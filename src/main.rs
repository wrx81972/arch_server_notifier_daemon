use reqwest;
use tokio::time::{sleep, Duration};
use std::collections::HashMap;


#[tokio::main]
async fn main() {
    let servers = vec![
        "https://archlinux.org/",
        "https://aur.archlinux.org/",
    ];

    let mut status_map: HashMap<String, bool> = HashMap::new();

    loop {
        for &url in &servers {
            let is_up = check_server(url).await;
            let previous_status = status_map.get(url).copied().unwrap_or(false);

            if is_up && !previous_status {
                println!("Server {} is up", url);
            }
            status_map.insert(url.to_string(), is_up);
        }
        sleep(Duration::from_secs(300)).await;
    }
}

async fn check_server(url: &str) -> bool {
    match reqwest::get(url).await {
        Ok(resp) => resp.status().is_success(),
        Err(_) => false,
    }
}