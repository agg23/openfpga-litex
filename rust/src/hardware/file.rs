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
                .MAIN
                .bridge_data_offset
                .write(|w| w.bits(data_offset));

            peripherals
                .MAIN
                .bridge_length
                .write(|w| w.bits(read_length));
            peripherals
                .MAIN
                .ram_data_address
                .write(|w| w.bits(storage_address));
            peripherals
                .MAIN
                .bridge_slot_id
                .write(|w| w.bits(bridge_slot_id));

            peripherals.MAIN.bridge_request_read.write(|w| w.bits(1));
        };
    }

    pub fn size(bridge_slot_id: u32) -> u32 {
        unsafe {
            let peripherals = pac::Peripherals::steal();

            peripherals
                .MAIN
                .bridge_slot_id
                .write(|w| w.bits(bridge_slot_id));

            peripherals.MAIN.bridge_file_size.read().bits()
        }
    }

    ///
    /// Returns true when operation complete, false when operation ongoing
    ///
    pub fn check_op_complete() -> bool {
        unsafe {
            let peripherals = pac::Peripherals::steal();

            peripherals.MAIN.bridge_status.read().bits() == 1
        }
    }

    pub fn block_op_complete() {
        while !File::check_op_complete() {
            // Loop
        }
    }
}
