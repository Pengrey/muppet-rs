use windows::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};

fn verify_ram() -> bool {
    let mut info = MEMORYSTATUSEX::default();
    info.dwLength = size_of::<MEMORYSTATUSEX>() as u32;

    unsafe {
        GlobalMemoryStatusEx(&mut info).expect("GlobalMemoryStatusEx Failed");

        info.ullTotalPhys > 2 * 1073741824 // 2G of RAM
    }
}

pub fn check_guardrails() -> bool {
    if !verify_ram() {
        #[cfg(feature = "debug")]
        println!("[-] Failed RAM check: System has less than 2 GiB of RAM.");

        return false;
    }

    true
}
