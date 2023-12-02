use litex_pac as pac;

pub struct File;

impl File {
    pub fn request_read(
        data_offset: u32,
        read_length: u32,
        storage_address: u32,
        bridge_slot_id: u32,
    ) {
        unsafe {
            let peripherals = pac::Peripherals::steal();

            peripherals
                .APF_BRIDGE
                .data_offset
                .write(|w| w.bits(data_offset));

            peripherals
                .APF_BRIDGE
                .transfer_length
                .write(|w| w.bits(read_length));
            peripherals
                .APF_BRIDGE
                .ram_data_address
                .write(|w| w.bits(storage_address));
            peripherals
                .APF_BRIDGE
                .slot_id
                .write(|w| w.bits(bridge_slot_id));

            peripherals.APF_BRIDGE.request_read.write(|w| w.bits(1));
        };
    }

    pub fn size(bridge_slot_id: u32) -> u32 {
        unsafe {
            let peripherals = pac::Peripherals::steal();

            peripherals
                .APF_BRIDGE
                .slot_id
                .write(|w| w.bits(bridge_slot_id));

            // Ensure slot change and size read has occured, as it takes several cycles
            peripherals.APF_BRIDGE.slot_id.read().bits();

            peripherals.APF_BRIDGE.file_size.read().bits()
        }
    }

    ///
    /// Returns true when operation complete, false when operation ongoing
    ///
    pub fn check_op_complete() -> bool {
        unsafe {
            let peripherals = pac::Peripherals::steal();

            peripherals.APF_BRIDGE.status.read().bits() == 1
        }
    }

    pub fn block_op_complete() {
        while !File::check_op_complete() {
            // Loop
        }
    }
}
