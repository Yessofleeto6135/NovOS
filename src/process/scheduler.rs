use spin::Mutex;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Clone, Debug)]
pub struct ProcessInfo {
    pub pid: usize,
    pub name: String,
    pub status: &'static str,
    pub cpu_percent: u8,
    pub mem_mb: usize,
}

pub static PROCESS_TABLE: Mutex<Vec<ProcessInfo>> = Mutex::new(Vec::new());

pub fn init_process_scheduler() {
    let mut table = PROCESS_TABLE.lock();
    table.clear();
    table.push(ProcessInfo { pid: 1, name: String::from("novos-kernel"), status: "Running", cpu_percent: 2, mem_mb: 8 });
    table.push(ProcessInfo { pid: 2, name: String::from("chromeos-wm"), status: "Running", cpu_percent: 4, mem_mb: 16 });
    table.push(ProcessInfo { pid: 3, name: String::from("system-tray"), status: "Running", cpu_percent: 1, mem_mb: 4 });
}

pub fn spawn_process(name: &str, mem: usize) -> usize {
    let mut table = PROCESS_TABLE.lock();
    let pid = table.len() + 1;
    table.push(ProcessInfo {
        pid,
        name: String::from(name),
        status: "Running",
        cpu_percent: 3,
        mem_mb: mem,
    });
    pid
}

pub fn kill_process(pid: usize) -> bool {
    let mut table = PROCESS_TABLE.lock();
    if let Some(pos) = table.iter().position(|p| p.pid == pid && p.pid > 3) {
        table.remove(pos);
        true
    } else {
        false
    }
}
