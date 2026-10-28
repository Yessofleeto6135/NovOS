use spin::Mutex;

pub struct EeePcHardware {
    pub is_eee_pc: bool,
    pub fn_keys_enabled: bool,
    pub backlight_level: u8, // 0..100
    pub battery_percent: u8,
    pub charging: bool,
}

pub static EEE_STATE: Mutex<EeePcHardware> = Mutex::new(EeePcHardware {
    is_eee_pc: true,
    fn_keys_enabled: true,
    backlight_level: 80,
    battery_percent: 92,
    charging: true,
});

pub fn init_eeepc_hardware() {
    let mut state = EEE_STATE.lock();
    state.is_eee_pc = true;
}
