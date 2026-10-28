use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;
use crate::process::scheduler::{PROCESS_TABLE, spawn_process, kill_process};
use crate::fs::vfs::{STORAGE, install_novos_to_disk};

pub static TERMINAL_HISTORY: Mutex<Vec<String>> = Mutex::new(Vec::new());

pub fn run_commander() {
    let mut hist = TERMINAL_HISTORY.lock();
    if hist.is_empty() {
        hist.push(String::from("NovOS Commander Shell v1.0"));
        hist.push(String::from("Type 'help' for available commands."));
        hist.push(String::from("novos@system:~$ "));
    }
}

pub fn execute_command(cmd: &str) -> String {
    let parts: Vec<&str> = cmd.trim().split_whitespace().collect();
    if parts.is_empty() {
        return String::from("");
    }

    match parts[0] {
        "help" => String::from(
            "NovOS Commander Commands:\n\
             ls               - List VFS files and directories\n\
             cat <file>       - Display file contents\n\
             ps               - List active running processes\n\
             top              - System performance and resource usage\n\
             kill <pid>       - Terminate process by PID\n\
             sysinfo          - Display kernel and hardware specifications\n\
             netstat          - Show network interface status\n\
             drvinfo          - Show detected hardware drivers (Win2K..Modern)\n\
             install <drive>  - Install NovOS persistently to HDD/SSD\n\
             launch <app>     - Launch a .nap application\n\
             echo <text>      - Echo text output"
        ),
        "ls" => {
            let storage = STORAGE.lock();
            let mut out = String::from("FILES IN VFS:\n");
            for file in storage.files.iter() {
                let kind = if file.is_dir { "<DIR>" } else { "<FILE>" };
                out.push_str(&alloc::format!("  {:6}  {:10}  {} bytes\n", kind, file.name, file.size));
            }
            out
        },
        "cat" => {
            if parts.len() < 2 {
                return String::from("Usage: cat <filename>");
            }
            let storage = STORAGE.lock();
            if let Some(file) = storage.files.iter().find(|f| f.name == parts[1]) {
                String::from_utf8_lossy(&file.content).into_owned()
            } else {
                alloc::format!("File not found: {}", parts[1])
            }
        },
        "ps" => {
            let table = PROCESS_TABLE.lock();
            let mut out = String::from("PID   PROCESS NAME         STATUS    CPU%   MEM\n");
            for p in table.iter() {
                out.push_str(&alloc::format!("{:4}  {:18}  {:8}  {:4}%   {} MB\n", p.pid, p.name, p.status, p.cpu_percent, p.mem_mb));
            }
            out
        },
        "top" => String::from("NovOS System Status: CPU 4% | Memory: 16MB/2048MB | Tasks: 6 | Uptime: 00:15:22"),
        "kill" => {
            if parts.len() < 2 {
                return String::from("Usage: kill <pid>");
            }
            if let Ok(pid) = parts[1].parse::<usize>() {
                if kill_process(pid) {
                    alloc::format!("Process {} terminated successfully.", pid)
                } else {
                    alloc::format!("Failed to kill process {}: Protected or not found.", pid)
                }
            } else {
                String::from("Invalid PID number.")
            }
        },
        "sysinfo" => String::from("NovOS Kernel Version 1.0\nArchitecture: x86 / x86_64\nBoot Mode: Live ISO / Multiboot VBE\nDisplay: 1024x768x32 True Color"),
        "netstat" => String::from("Interface: Intel e1000 / RTL8139 Ethernet\nIP: 192.168.1.105 | Subnet: 255.255.255.0 | WiFi SSID: NovOS_HighSpeed_WiFi"),
        "drvinfo" => String::from("Active Drivers:\n - Display: VBE Framebuffer Driver (1024x768)\n - Network: Intel e1000 PCI & Realtek RTL8139\n - USB: xHCI / EHCI Host Controller\n - Bluetooth: NovOS BT Stack v1.0\n - Netbook: Asus Eee PC ACPI & Fn-Keys Driver"),
        "install" => {
            let target = if parts.len() > 1 { parts[1] } else { "/dev/sda1" };
            if install_novos_to_disk(target) {
                alloc::format!("Successfully installed NovOS persistently to {}!", target)
            } else {
                String::from("Installation failed.")
            }
        },
        "launch" => {
            if parts.len() < 2 {
                return String::from("Usage: launch <app_name>");
            }
            let pid = spawn_process(parts[1], 12);
            alloc::format!("Launched {} as Process ID {}.", parts[1], pid)
        },
        "echo" => parts[1..].join(" "),
        _ => alloc::format!("Unknown command: '{}'. Type 'help' for commands.", parts[0]),
    }
}
