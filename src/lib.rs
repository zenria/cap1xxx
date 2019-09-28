//! cap1xxx [`embedded-hal`] I2C driver for Microchip cap1xxx capacitive touch buttons
//!
//! This crate is primarily targeting cap1666 6 buttons - 6 led hardware found
//! on the Pimoroni GFX HAT

use embedded_hal::blocking::i2c::{Read, Write};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
