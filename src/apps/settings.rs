use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;

#[derive(Clone, Debug)]
pub struct SettingsCategory {
    pub name: &'static str,
    pub icon: &'static str,
    pub options: Vec<SettingItem>,
}

#[derive(Clone, Debug)]
pub struct SettingItem {
    pub title: &'static str,
    pub value: String,
    pub is_toggle: bool,
    pub enabled: bool,
}

pub static SETTINGS_STATE: Mutex<Vec<SettingsCategory>> = Mutex::new(Vec::new());

pub fn run_settings() {
    let mut state = SETTINGS_STATE.lock();
    state.clear();

    state.push(SettingsCategory {
        name: "Network & Internet",
        icon: "WiFi",
        options: alloc::vec![
            SettingItem { title: "Wi-Fi Status", value: String::from("Connected (NovOS_HighSpeed)"), is_toggle: true, enabled: true },
            SettingItem { title: "Ethernet Adapter", value: String::from("Intel e1000 / RTL8139 Active"), is_toggle: false, enabled: true },
            SettingItem { title: "IP Address", value: String::from("192.168.1.105"), is_toggle: false, enabled: true },
        ],
    });

    state.push(SettingsCategory {
        name: "Bluetooth & Devices",
        icon: "BT",
        options: alloc::vec![
            SettingItem { title: "Bluetooth Adapter", value: String::from("Active & Discoverable"), is_toggle: true, enabled: true },
            SettingItem { title: "Paired Device", value: String::from("NovOS Wireless Controller"), is_toggle: false, enabled: true },
        ],
    });

    state.push(SettingsCategory {
        name: "Display & Appearance",
        icon: "Disp",
        options: alloc::vec![
            SettingItem { title: "Screen Resolution", value: String::from("1024x768 (VESA VBE / GOP)"), is_toggle: false, enabled: true },
            SettingItem { title: "Dark Mode Theme", value: String::from("ChromeOS Dark"), is_toggle: true, enabled: true },
            SettingItem { title: "Asus Eee PC Backlight", value: String::from("80% Brightness"), is_toggle: false, enabled: true },
        ],
    });

    state.push(SettingsCategory {
        name: "Linux Support Mode",
        icon: "Linux",
        options: alloc::vec![
            SettingItem { title: "Enable Linux Subsystem", value: String::from("Active (.deb / APT support)"), is_toggle: true, enabled: true },
            SettingItem { title: "Terminal Shell Integration", value: String::from("Commander Shell Linked"), is_toggle: true, enabled: true },
        ],
    });

    state.push(SettingsCategory {
        name: "System & Installer",
        icon: "Sys",
        options: alloc::vec![
            SettingItem { title: "OS Version", value: String::from("NovOS v1.0 Enterprise (x86/x86_64)"), is_toggle: false, enabled: true },
            SettingItem { title: "System Memory", value: String::from("16 MB Heap / 2048 MB RAM"), is_toggle: false, enabled: true },
            SettingItem { title: "Install NovOS to Disk", value: String::from("Live ISO Session - Click to Install"), is_toggle: false, enabled: true },
        ],
    });
}
