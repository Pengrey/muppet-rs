use serde::Deserialize;
use std::fs;
use std::path::Path;

#[macro_export]
macro_rules! info {
    ($content:expr) => {
        println!("cargo::warning=\r[{}] {}", "\x1b[34m>\x1b[0m", $content);
    };
}

#[derive(Deserialize)]
struct Config {
    target_url: String,
    exfil_domain: String,
    killdate_timestamp: u64,
}

fn main() {
    println!("cargo:rerun-if-changed=config.toml");
    println!("cargo:rerun-if-changed=build.rs");

    let config_path = Path::new("config.toml");
    let config_str = fs::read_to_string(config_path)
        .expect("Failed to read config.toml");

    let config: Config = toml::from_str(&config_str)
        .expect("Failed to parse config.toml");

    info!(format!("Using target url: {}", config.target_url));
    println!("cargo:rustc-env=TARGET_URL={}", config.target_url);

    info!(format!("Using exfil domain: {}", config.exfil_domain));
    println!("cargo:rustc-env=EXFIL_DOMAIN={}", config.exfil_domain);

    info!(format!("Using killdate timestamp: {}", config.killdate_timestamp));
    println!("cargo:rustc-env=KILLDATE_TIMESTAMP={}", config.killdate_timestamp);
}
