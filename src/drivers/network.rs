use spin::Mutex;

#[derive(Debug, Clone)]
pub struct NetworkStats {
    pub connected: bool,
    pub adapter_name: &'static str,
    pub mac_addr: [u8; 6],
    pub ip_addr: [u8; 4],
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub wifi_ssid: &'static str,
    pub wifi_signal: u8,
}

pub static NET_STATE: Mutex<NetworkStats> = Mutex::new(NetworkStats {
    connected: true,
    adapter_name: "Intel e1000 / RTL8139 Ethernet",
    mac_addr: [0x52, 0x54, 0x00, 0x12, 0x34, 0x56],
    ip_addr: [192, 168, 1, 105],
    rx_bytes: 1048576,
    tx_bytes: 524288,
    wifi_ssid: "NovOS_HighSpeed_WiFi",
    wifi_signal: 88,
});

pub fn init_network_adapters() {
    let mut state = NET_STATE.lock();
    state.connected = true;
}
