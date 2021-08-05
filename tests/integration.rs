use embedded_hal_mock::common::Generic;
use embedded_hal_mock::delay::MockNoop;
use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTransaction};
use lcd_1602_i2c::Lcd;
use std::vec;

const EXPECTED_ADDRESS: u8 = 123;
const RGB_ADDRESS: u8 = 34;

#[test]
fn sample() {
    let _ = new_lcd(EXPECTED_ADDRESS, RGB_ADDRESS);
}

fn new_lcd(address: u8, rgb_address: u8) -> Lcd<Generic<I2cTransaction>> {
    let mut delay = MockNoop::new();

    let mut expectations = Expectations::new();
    expectations
        // Initial command sequence for "wakeup"
        .command_bytes(address, 0x28)
        .command_bytes(address, 0x28)
        .command_bytes(address, 0x28)

        // Display On
        .command_bytes(address, 8 | 4)

        // Clear
        .command_bytes(address, 0x01)

        // Set LCD mode
        .command_bytes(address, 0x02 | 0x04)

        // Initialize the RGB backlight
        .reg_bytes(rgb_address, 0, 0)
        .reg_bytes(rgb_address, 8, 0xFF)
        .reg_bytes(rgb_address, 1, 0x20)
    ;

    let i2c = I2cMock::new(expectations.as_array());

    Lcd::new(i2c, address, rgb_address, &mut delay).unwrap()
}

struct Expectations {
    expectations: Vec<I2cTransaction>,
}

impl Expectations {
    pub fn new() -> Self {
        Expectations { expectations: Vec::new() }
    }

    pub fn command_bytes(&mut self, address: u8, byte: u8) -> &mut Self {
        self.expectations.push(I2cTransaction::write(address, vec![0x80, byte]));
        self
    }

    fn reg_bytes(&mut self, address: u8, reg: u8, byte: u8) -> &mut Self {
        self.expectations.push(I2cTransaction::write(address, vec![reg, byte]));
        self
    }
    
    pub fn as_array(&self) -> &[I2cTransaction] {
        self.expectations.as_slice()
    }
}
