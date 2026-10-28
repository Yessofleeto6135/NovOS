use alloc::vec::Vec;
use spin::Mutex;
use crate::process::scheduler::{PROCESS_TABLE, ProcessInfo};

#[derive(Clone, Debug)]
pub struct HardwareStats {
    pub cpu_usage_percent: u8,
    pub ram_used_mb: usize,
    pub ram_total_mb: usize,
    pub disk_usage_mb: usize,
    pub disk_total_mb: usize,
    pub uptime_seconds: u64,
}

pub static TASK_MANAGER_STATE: Mutex<HardwareStats> = Mutex::new(HardwareStats {
    cpu_usage_percent: 7,
    ram_used_mb: 28,
    ram_total_mb: 2048,
    disk_usage_mb: 120,
    disk_total_mb: 32768,
    uptime_seconds: 924,
});

pub fn run_task_manager() {}

pub fn get_live_task_list() -> Vec<ProcessInfo> {
    let table = PROCESS_TABLE.lock();
    table.clone()
}

pub fn get_hardware_diagnostics() -> HardwareStats {
    let stats = TASK_MANAGER_STATE.lock();
    stats.clone()
}
