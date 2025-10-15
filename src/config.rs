use std::fs;
use std::io::Write;
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub check_interval_seconds: u64,
    pub servers: Vec<String>,
}

pub fn default_config_content() -> &'static str {
    r#"
check_interval_seconds = 300

servers = [
    "https://archlinux.org/",
    "https://aur.archlinux.org/"
]
"#
}

pub fn get_config_path() -> PathBuf {
    let mut config_dir = dirs::config_dir().expect("Cannot find config dir");
    config_dir.push("arch_update_notifier");
    fs::create_dir_all(&config_dir).expect("Cannot create config dir");
    config_dir.push("config.toml");
    config_dir
}

pub fn load_or_create_config(path: &PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
    if !path.exists() {
        let mut file = fs::File::create(path)?;
        file.write_all(default_config_content().as_bytes())?;
        println!("Config file not found, therefore new one has been created: {:?}", path);
    }
    let contents = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}