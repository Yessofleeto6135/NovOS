use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;
use crate::fs::vfs::{STORAGE, FileNode};

#[derive(Clone, Debug)]
pub struct DragDropPayload {
    pub file_name: String,
    pub content: Vec<u8>,
    pub source_app: String,
    pub is_dragging: bool,
}

pub static ACTIVE_DRAG_DROP: Mutex<Option<DragDropPayload>> = Mutex::new(None);

pub fn run_files() {}

pub fn get_directory_files() -> Vec<FileNode> {
    let storage = STORAGE.lock();
    storage.files.clone()
}

pub fn start_drag_file(file_name: &str) {
    let storage = STORAGE.lock();
    if let Some(file) = storage.files.iter().find(|f| f.name == file_name) {
        *ACTIVE_DRAG_DROP.lock() = Some(DragDropPayload {
            file_name: file.name.clone(),
            content: file.content.clone(),
            source_app: String::from("NovOS Files"),
            is_dragging: true,
        });
    }
}

pub fn drop_file_to_target(target_app: &str) -> Option<String> {
    let mut payload = ACTIVE_DRAG_DROP.lock();
    if let Some(ref mut p) = *payload {
        p.is_dragging = false;
        let msg = alloc::format!("Dropped file '{}' into {}", p.file_name, target_app);
        *payload = None;
        Some(msg)
    } else {
        None
    }
}
