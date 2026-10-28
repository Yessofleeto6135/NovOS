use spin::Mutex;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Clone, Debug)]
pub struct FileNode {
    pub name: String,
    pub is_dir: bool,
    pub size: usize,
    pub content: Vec<u8>,
}

pub struct SystemStorage {
    pub is_live_mode: bool,
    pub target_disk_installed: bool,
    pub disk_label: String,
    pub files: Vec<FileNode>,
}

pub static STORAGE: Mutex<SystemStorage> = Mutex::new(SystemStorage {
    is_live_mode: true,
    target_disk_installed: false,
    disk_label: String::new(),
    files: Vec::new(),
});

pub fn init_vfs() {
    let mut storage = STORAGE.lock();
    storage.is_live_mode = true;
    storage.disk_label = String::from("NovOS Live ISO (RAM Drive)");

    storage.files.push(FileNode {
        name: String::from("Welcome_to_NovOS.txt"),
        is_dir: false,
        size: 53,
        content: String::from("Welcome to NovOS live session! You can try or install.").into_bytes(),
    });
    storage.files.push(FileNode {
        name: String::from("System_Info.txt"),
        is_dir: false,
        size: 42,
        content: String::from("NovOS Kernel Version 1.0 (x86 / x86_64)").into_bytes(),
    });
    storage.files.push(FileNode {
        name: String::from("Install_NovOS.nap"),
        is_dir: false,
        size: 1024,
        content: String::from("NovOS Persistent HDD/SSD Installer").into_bytes(),
    });
    storage.files.push(FileNode {
        name: String::from("Documents"),
        is_dir: true,
        size: 0,
        content: Vec::new(),
    });
}

pub fn install_novos_to_disk(target_drive: &str) -> bool {
    let mut storage = STORAGE.lock();
    storage.is_live_mode = false;
    storage.target_disk_installed = true;
    storage.disk_label = String::from(target_drive);
    true
}
