use winreg::enums::{HKEY_CURRENT_USER};
use winreg::RegKey;

pub fn read_notification_interval() -> u32 {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    hkcu.create_subkey("SOFTWARE\\DevDuFutur\\ChuckNorust").and_then(|(key, _)|
        key.get_value("NotificationInterval")
    ).unwrap_or(10000u32)
}

pub fn write_notification_interval(interval: &u32) {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok((key, _)) = hkcu.create_subkey("SOFTWARE\\DevDuFutur\\ChuckNorust") {
        key.set_value("NotificationInterval", interval).unwrap_or(()); // fails silently
    }
}