use winreg::enums::{HKEY_CURRENT_USER};
use winreg::RegKey;
use winreg::types::{FromRegValue, ToRegValue};
use std::ffi::OsStr;

fn read_value<T: FromRegValue, N: AsRef<OsStr>>(key_name: N, default: T) -> T {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    hkcu.create_subkey("SOFTWARE\\DevDuFutur\\ChuckNorust").and_then(|(key, _)|
        key.get_value(key_name)
    ).unwrap_or(default)
}

fn write_value<T: ToRegValue, N: AsRef<OsStr>>(key_name: N, value: T) {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok((key, _)) = hkcu.create_subkey("SOFTWARE\\DevDuFutur\\ChuckNorust") {
        key.set_value(key_name, &value).unwrap_or(()); // fails silently
    }
}

pub fn read_notification_interval() -> u32 {
    read_value("NotificationInterval", 10000u32)
}

pub fn read_silent_notification() -> bool {
    read_value("SilentNotification", 1u32) == 1u32
}

pub fn write_notification_interval(interval: u32) {
    write_value("NotificationInterval", interval);
}

pub fn write_silent_notification(silent: bool) {
    write_value("SilentNotification", if silent { 1u32 } else { 0u32 });
}