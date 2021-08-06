use embedded_hal_mock::common::Generic;
use embedded_hal_mock::delay::MockNoop;
use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTransaction};
use lcd_1602_i2c::{Lcd, Cursor, LcdDisplay, Blink};
use std::vec;

const BLINK_ON: u8 = 0x01;
const CURSOR_ON: u8 = 0x02;
const DISPLAY_ON: u8 = 0x04;
const DISPLAY_CONTROL: u8 = 0x08;
const EXPECTED_ADDRESS: u8 = 123;
const RGB_ADDRESS: u8 = 34;

// Ensure the initialization sequence doesn't break
#[test]
fn lcd_initialization() {
    let expectations = lcd_expectations(EXPECTED_ADDRESS, RGB_ADDRESS);
    let _ = new_lcd(&expectations, EXPECTED_ADDRESS, RGB_ADDRESS);
}

#[test]
fn display_off() {
    // Arrange
    let expectations =
        lcd_expectations(EXPECTED_ADDRESS, RGB_ADDRESS)
        .command_bytes(EXPECTED_ADDRESS, DISPLAY_CONTROL);

    let mut lcd = new_lcd(&expectations, EXPECTED_ADDRESS, RGB_ADDRESS);

    // Act
    let _ = lcd.set_display(LcdDisplay::Off);
}

#[test]
fn cursor_on() {
    // Arrange
    let expectations =
        lcd_expectations(EXPECTED_ADDRESS, RGB_ADDRESS)
        .command_bytes(EXPECTED_ADDRESS, DISPLAY_CONTROL | DISPLAY_ON | CURSOR_ON);

    let mut lcd = new_lcd(&expectations, EXPECTED_ADDRESS, RGB_ADDRESS);

    // Act
    let _ = lcd.set_cursor(Cursor::On);
}

#[test]
fn cursor_off() {
    // Arrange
    let expectations =
        lcd_expectations(EXPECTED_ADDRESS, RGB_ADDRESS)
        .command_bytes(EXPECTED_ADDRESS, DISPLAY_CONTROL | DISPLAY_ON | CURSOR_ON)
        .command_bytes(EXPECTED_ADDRESS, DISPLAY_CONTROL | DISPLAY_ON);

    let mut lcd = new_lcd(&expectations, EXPECTED_ADDRESS, RGB_ADDRESS);
    let _ = lcd.set_cursor(Cursor::On);

    // Act
    let _ = lcd.set_cursor(Cursor::Off);
}

#[test]
fn blink_on() {
    // Arrange
    let expectations =
        lcd_expectations(EXPECTED_ADDRESS, RGB_ADDRESS)
        .command_bytes(EXPECTED_ADDRESS, DISPLAY_CONTROL | DISPLAY_ON | BLINK_ON);

    let mut lcd = new_lcd(&expectations, EXPECTED_ADDRESS, RGB_ADDRESS);

    // Act
    let _ = lcd.set_blink(Blink::On);
}

#[test]
fn blink_off() {
    // Arrange
    let expectations =
        lcd_expectations(EXPECTED_ADDRESS, RGB_ADDRESS)
        .command_bytes(EXPECTED_ADDRESS, DISPLAY_CONTROL | DISPLAY_ON | BLINK_ON)
        .command_bytes(EXPECTED_ADDRESS, DISPLAY_CONTROL | DISPLAY_ON);

    let mut lcd = new_lcd(&expectations, EXPECTED_ADDRESS, RGB_ADDRESS);
    let _ = lcd.set_blink(Blink::On);

    // Act
    let _ = lcd.set_blink(Blink::Off);
}

fn new_lcd<'a>(
    expectations: &'a Expectations,
    address: u8,
    rgb_address: u8,
) -> Lcd<Generic<I2cTransaction>> {
    let mut delay = MockNoop::new();
    let i2c = I2cMock::new(expectations.as_array());
    Lcd::new(i2c, address, rgb_address, &mut delay).unwrap()
}

// Returns expectations for the initialization of the LCD display that is always
// via new().
fn lcd_expectations(address: u8, rgb_address: u8) -> Expectations {
    Expectations::new()
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
}

struct Expectations {
    expectations: Vec<I2cTransaction>,
}

impl Expectations {
    pub fn new() -> Self {
        Expectations {
            expectations: Vec::new(),
        }
    }

    pub fn command_bytes(mut self, address: u8, byte: u8) -> Self {
        self.expectations
            .push(I2cTransaction::write(address, vec![0x80, byte]));
        self
    }

    fn reg_bytes(mut self, address: u8, reg: u8, byte: u8) -> Self {
        self.expectations
            .push(I2cTransaction::write(address, vec![reg, byte]));
        self
    }

    pub fn as_array(&self) -> &[I2cTransaction] {
        self.expectations.as_slice()
    }
}
