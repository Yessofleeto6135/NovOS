#[derive(Debug, Clone, Copy)]
pub struct PciDevice {
    pub bus: u8,
    pub slot: u8,
    pub func: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class_code: u8,
    pub subclass: u8,
}

pub static mut DISCOVERED_DEVICES: [Option<PciDevice>; 32] = [None; 32];
pub static mut DEVICE_COUNT: usize = 0;

pub fn scan_pci_bus() {
    // Scan PCI configuration space across buses 0..4
    for bus in 0..4 {
        for slot in 0..32 {
            let vendor_id = pci_read_word(bus, slot, 0, 0);
            if vendor_id != 0xFFFF {
                let device_id = pci_read_word(bus, slot, 0, 2);
                let class_word = pci_read_word(bus, slot, 0, 10);
                let class_code = (class_word >> 8) as u8;
                let subclass = (class_word & 0xFF) as u8;

                unsafe {
                    if DEVICE_COUNT < 32 {
                        DISCOVERED_DEVICES[DEVICE_COUNT] = Some(PciDevice {
                            bus, slot, func: 0, vendor_id, device_id, class_code, subclass
                        });
                        DEVICE_COUNT += 1;
                    }
                }
            }
        }
    }
}

fn pci_read_word(bus: u8, slot: u8, func: u8, offset: u8) -> u16 {
    let address = ((bus as u32) << 16)
        | ((slot as u32) << 11)
        | ((func as u32) << 8)
        | ((offset as u32) & 0xFC)
        | 0x80000000;

    unsafe {
        outl(0xCF8, address);
        ((inl(0xCFC) >> ((offset & 2) * 8)) & 0xFFFF) as u16
    }
}

unsafe fn outl(port: u16, val: u32) {
    core::arch::asm!("out dx, eax", in("dx") port, in("eax") val);
}

unsafe fn inl(port: u16) -> u32 {
    let res: u32;
    core::arch::asm!("in eax, dx", out("eax") res, in("dx") port);
    res
}
