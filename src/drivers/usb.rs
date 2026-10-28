use spin::Mutex;

pub struct UsbHostController {
    pub controller_type: &'static str, // EHCI, UHCI, XHCI
    pub connected_devices: u8,
}

pub static USB_STATE: Mutex<UsbHostController> = Mutex::new(UsbHostController {
    controller_type: "xHCI / EHCI USB Controller",
    connected_devices: 3, // Keyboard, Mouse, Storage
});

pub fn init_usb_controllers() {
    let mut state = USB_STATE.lock();
    state.connected_devices = 3;
}
