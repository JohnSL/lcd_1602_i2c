//! # lcd_1602_i2c
//!
//! Provides a driver for common 16x2 LCD displays that use the HD44780 chip to
//! drive the display, and an I2C chip that connects to teh HD44780.
//!
//! This is a basic implementation, and doesn't currently support custom characters.

#![no_std]
use embedded_hal::blocking::{i2c, delay::DelayMs};

mod display_control;
use display_control::{DisplayControl, ControlOptions};

pub struct Lcd<I>
where
    I: i2c::Write,
{
    i2c: I,
    show_function: u8,
    control: DisplayControl,
}

impl<I> Lcd<I>
where
    I: i2c::Write
    {
    /// Creates a new instance of the display object.
    ///
    /// ```rust
    /// let lcd = Lcd::new(i2c_bus, &mut delay);
    /// ```
    pub fn new<D>(i2c: I, delay: &mut D) -> Result<Self, <I as i2c::Write>::Error>
    where
        D: DelayMs<u16>
    {
        const LCD_4BITMODE: u8 = 0x00;
        const LCD_2LINE: u8 = 0x08;
        const LCD_5X8_DOTS: u8 = 0x00;

        let mut display = Lcd {
            i2c: i2c,
            show_function: LCD_4BITMODE | LCD_2LINE | LCD_5X8_DOTS,
            control: DisplayControl::new()
        };
        display.init(delay)?;
        Ok(display)
    }

    //
    // Initialize the display for the first time after power up
    //
    pub fn init<D>(&mut self, delay: &mut D) -> Result<(), <I as i2c::Write>::Error>
    where D: DelayMs<u16> {
        const LCD_FUNCTIONSET: u8 = 0x20;

        delay.delay_ms(80); // Need to wait at least 40ms before sending commands

        // Send the initial command sequence according to the HD44780 datasheet
        self.command(LCD_FUNCTIONSET | self.show_function)?;
        delay.delay_ms(5);

        self.command(LCD_FUNCTIONSET | self.show_function)?;
        delay.delay_ms(5);

        self.command(LCD_FUNCTIONSET | self.show_function)?;

        self.set_control_option(ControlOptions::DisplayOn)?;

        self.clear(delay)?;

        // Display entry mode
        const LCD_ENTRYLEFT: u8 = 0x02;
        const LCD_ENTRYSHIFTDECREMENT: u8 = 0x00;
        const LCD_ENTRYMODESET: u8 = 0x04;

        self.command(LCD_ENTRYMODESET | LCD_ENTRYLEFT | LCD_ENTRYSHIFTDECREMENT)?;

        // Initialize the backlight
        const REG_MODE1: u8     = 0x00;
        const REG_MODE2: u8     = 0x01;
        const REG_OUTPUT: u8    = 0x08;
    
        self.set_reg(REG_MODE1, 0)?;

        // Set the LEDs controllable by both PWM and GRPPWM registers
        self.set_reg(REG_OUTPUT, 0xFF)?;
        self.set_reg(REG_MODE2, 0x20)?;

        self.set_rgb(255, 255, 255)
    }

    // Clear the display
    pub fn clear(&mut self, delay: &mut dyn DelayMs<u16>) -> Result<(), <I as i2c::Write>::Error> {
        const LCD_CLEARDISPLAY: u8 = 0x01;

        let result = self.command(LCD_CLEARDISPLAY);
        delay.delay_ms(2);
        result
    }

    // Set the position of the cursor
    pub fn cursor_position(&mut self, x: u8, y: u8) -> Result<(), <I as i2c::Write>::Error> {
        let col = if y == 0_u8 { x | 0x80 } else { x | 0xC0 };
        self.command(col)
    }

    // Turns on the cursor, which is a non-blinking _
    pub fn cursor_on(&mut self) -> Result<(), <I as i2c::Write>::Error> {
        self.set_control_option(ControlOptions::CursorOn)
    }

    pub fn cursor_off(&mut self) -> Result<(), <I as i2c::Write>::Error> {
        self.clear_control_option(ControlOptions::CursorOn)
    }

    pub fn blink_on(&mut self) -> Result<(), <I as i2c::Write>::Error> {
        self.set_control_option(ControlOptions::BlinkOn)
    }

    pub fn send_char(&mut self, char: char) -> Result<(), <I as i2c::Write>::Error> {
        self.send_two(0x40, char as u8)
    }

    pub fn print(&mut self, s: &str) -> Result<(), <I as i2c::Write>::Error> {
        for c in s.chars() {
            self.send_char(c)?;
        }

        Ok(())
    }

    // Send a command to the LCD display
    fn command(&mut self, value: u8) -> Result<(), <I as i2c::Write>::Error> {
        self.send_two(0x80, value)
    }

    fn send_two(&mut self, byte1: u8, byte2: u8) -> Result<(), <I as i2c::Write>::Error> {
        let result = self.i2c.write(LCD_ADDRESS, &[byte1, byte2]);
        result
    }

    fn set_reg(&mut self, addr: u8, data: u8) -> Result<(), <I as i2c::Write>::Error> {
        self.i2c.write(RGB_ADDRESS, &[addr, data])
    }

    // Set the color of the backlight
    fn set_rgb(&mut self, r: u8, g: u8, b: u8) -> Result<(), <I as i2c::Write>::Error> {
        const REG_RED: u8       = 0x04;        // pwm2
        const REG_GREEN: u8     = 0x03;        // pwm1
        const REG_BLUE: u8      = 0x02;        // pwm0
    
        self.set_reg(REG_RED, r)?;
        self.set_reg(REG_GREEN, g)?;
        self.set_reg(REG_BLUE, b)
    }

    fn set_control_option(&mut self, option: ControlOptions) -> Result<(), <I as i2c::Write>::Error> {
        const LCD_DISPLAYCONTROL: u8 = 0x08;

        self.control.set(option);
        let value = self.control.value();
        self.command(LCD_DISPLAYCONTROL | value)
    }

    fn clear_control_option(&mut self, option: ControlOptions) -> Result<(), <I as i2c::Write>::Error> {
        const LCD_DISPLAYCONTROL: u8 = 0x08;

        self.control.clear(option);
        let value = self.control.value();
        self.command(LCD_DISPLAYCONTROL | value)
    }
}

// Device I2c addresses
const LCD_ADDRESS: u8 = 0x7c >> 1;
const RGB_ADDRESS: u8 = 0xc0 >> 1;
