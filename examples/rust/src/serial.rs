use embedded_hal::serial::Write;
use nb;

pub struct Serial;

const ADDRESS: *mut u8 = 0x8000_0000 as *mut u8;

impl Write<u8> for Serial {
    type Error = ();

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        unsafe { *ADDRESS = word };

        Ok(())
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        Ok(())
    }
}
