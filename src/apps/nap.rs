use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;

#[derive(Clone, Debug)]
pub struct NapHeader {
    pub magic: [u8; 4], // "NAP1"
    pub app_name: String,
    pub version: String,
    pub app_type: NapAppType,
    pub icon_color: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NapAppType {
    Settings,
    Commander,
    Browser,
    Files,
    CraftBlocks,
    TaskManager,
    Custom(String),
}

#[derive(Clone, Debug)]
pub struct NapPackage {
    pub header: NapHeader,
    pub executable_code: Vec<u8>,
}

pub static INSTALLED_NAPS: Mutex<Vec<NapPackage>> = Mutex::new(Vec::new());

pub fn init_nap_engine() {
    let mut apps = INSTALLED_NAPS.lock();
    apps.clear();

    apps.push(create_nap_package("Settings", "1.0.0", NapAppType::Settings, 0xFF3B82F6));
    apps.push(create_nap_package("Commander", "1.0.0", NapAppType::Commander, 0xFF10B981));
    apps.push(create_nap_package("Browser", "1.0.0", NapAppType::Browser, 0xFFEF4444));
    apps.push(create_nap_package("Files", "1.0.0", NapAppType::Files, 0xFFF59E0B));
    apps.push(create_nap_package("CraftBlocks 3D", "1.0.0", NapAppType::CraftBlocks, 0xFF8B5CF6));
    apps.push(create_nap_package("Task Manager", "1.0.0", NapAppType::TaskManager, 0xFF6366F1));
}

fn create_nap_package(name: &str, ver: &str, app_type: NapAppType, color: u32) -> NapPackage {
    NapPackage {
        header: NapHeader {
            magic: *b"NAP1",
            app_name: String::from(name),
            version: String::from(ver),
            app_type,
            icon_color: color,
        },
        executable_code: Vec::new(),
    }
}

pub fn load_nap_app(nap: &NapPackage) {
    // Executes .nap package entry routine in window runtime context
}
