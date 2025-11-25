use reqwest;
use tokio::time::{sleep, Duration};
use std::collections::HashMap;
use crate::config::Config;

pub async fn monitor(config: &Config) {
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
    let max_tries = 3;
    let retry_delay = Duration::from_secs(5);

    for attempt in 1..=max_tries {
        match reqwest::get(url).await {
            Ok(resp) => {
                if resp.status().is_success() {
                    return true;
                } else {
                    eprintln!("Server {} appears to be offline: {}", url, resp.status());
                }
            }
            Err(e) => {
                eprintln!("{} attempt failed. Server: {}, code: {}", attempt, url, e);
            }
        }
        sleep(retry_delay).await;
    }
    false

    // match reqwest::get(url).await {
    //     Ok(resp) => resp.status().is_success(),
    //     Err(_) => false,
    // }
}

// TESTY JEDNOSTKOWE
#[cfg(test)]
mod tests {
    use super::*;
    use tokio; // potrzebne do #[tokio::test]

    // bardzo prosty przykład - na początek test "techniczny"
    #[tokio::test]
    async fn dummy_test_runs() {
        assert_eq!(2 + 2, 4);
    }
}
