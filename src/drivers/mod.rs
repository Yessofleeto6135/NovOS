pub mod display;
pub mod pci;
pub mod network;
pub mod usb;
pub mod bluetooth;
pub mod eeepc;
pub mod input;

pub fn init_all() {
    display::init_framebuffer();
    pci::scan_pci_bus();
    network::init_network_adapters();
    usb::init_usb_controllers();
    bluetooth::init_bluetooth_adapter();
    eeepc::init_eeepc_hardware();
    input::init_input_devices();
}
