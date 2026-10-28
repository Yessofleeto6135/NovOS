use spin::Mutex;

pub struct BluetoothState {
    pub enabled: bool,
    pub connected_device: Option<&'static str>,
}

pub static BT_STATE: Mutex<BluetoothState> = Mutex::new(BluetoothState {
    enabled: true,
    connected_device: Some("NovOS Wireless Audio / Mouse"),
});

pub fn init_bluetooth_adapter() {
    let mut state = BT_STATE.lock();
    state.enabled = true;
}
