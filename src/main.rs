#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;

pub mod drivers;
pub mod memory;
pub mod fs;
pub mod gui;
pub mod apps;
pub mod process;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    novos_init();
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    novos_init();
    0
}

pub fn novos_init() {
    drivers::init_all();
    memory::init_heap();
    fs::vfs::init_vfs();
    process::scheduler::init_process_scheduler();
    apps::nap::init_nap_engine();
    apps::settings::run_settings();
    apps::commander::run_commander();
    apps::browser::run_browser();
    apps::craftblocks::run_craftblocks();

    // Launch NovOS Graphical Desktop Environment
    gui::desktop::launch_novos_desktop();
    gui::desktop::update_desktop_loop();
}

#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.add(i) = *src.add(i);
        i += 1;
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memset(dest: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.add(i) = c as u8;
        i += 1;
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let a = *s1.add(i);
        let b = *s2.add(i);
        if a != b {
            return a as i32 - b as i32;
        }
        i += 1;
    }
    0
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}
