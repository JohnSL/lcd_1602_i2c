/*!
# Platform-agnostic driver for I2C 16x2 character displays

Provides a driver for common 16x2 LCD displays that use the AiP31068L chip to
drive the display, and a PCA9633 chip to drive the RGB backlight.

This is a basic implementation, and doesn't currently support custom characters.

This has been tested with the [Waveshare LCD1602 module](https://www.waveshare.com/wiki/LCD1602_RGB_Module).
It may also work with other RGB displays like the [Groove 16X2 LDC RGB](https://www.seeedstudio.com/Grove-LCD-RGB-Backlight-p-1643.html)
*/

#![no_std]
use embedded_hal::blocking::{i2c, delay::DelayMs};

mod display_control;
use display_control::{DisplayControl};

pub use display_control::{Cursor, LcdDisplay, Blink};

/**
Handles all the logic related to working with the character LCD via I2C. You'll
need to create an instance of this with the `new()` method.

The `I` generic type needs to implement the `embedded_hal::blocking::Write` trait.
*/
pub struct Lcd<I>
where
    I: i2c::Write,
{
    i2c: I,
    show_function: u8,
    control: DisplayControl,
    address: u8,
    rgb_address: u8,
}

impl<I> Lcd<I>
where
    I: i2c::Write
    {
    /**
    Creates a new instance of the display object.

    # Example

    ```rust
    let lcd = Lcd::new(i2c_bus, address, rgb_address, &mut delay);
    ```

    `i2c` needs to implement the `embedded_hal::blocking::Write` trait.

    `delay` needs to implement the `embedded_hal::blocking::delay::DelayMs` trait.

    # Errors

    The I2C library will return an error if it's not able to write to the device.
    This is always a trait of type `embedded_hal::blocking::Write::Error` that
    is implemented by the I2C instance.
    */
    pub fn new<D>(i2c: I, address: u8, rgb_address: u8, delay: &mut D) -> Result<Self, <I as i2c::Write>::Error>
    where
        D: DelayMs<u16>
    {
        const LCD_4BITMODE: u8 = 0x00;
        const LCD_2LINE: u8 = 0x08;
        const LCD_5X8_DOTS: u8 = 0x00;

        let mut display = Lcd {
            i2c,
            show_function: LCD_4BITMODE | LCD_2LINE | LCD_5X8_DOTS,
            control: DisplayControl::new(),
            address,
            rgb_address,
        };
        display.init(delay)?;
        Ok(display)
    }

    // Initialize the display for the first time after power up
    fn init<D>(&mut self, delay: &mut D) -> Result<(), <I as i2c::Write>::Error>
    where D: DelayMs<u16> {
        const LCD_FUNCTIONSET: u8 = 0x20;

        delay.delay_ms(80); // Need to wait at least 40ms before sending commands

        // Send the initial command sequence according to the HD44780 datasheet
        self.command(LCD_FUNCTIONSET | self.show_function)?;
        delay.delay_ms(5);

        self.command(LCD_FUNCTIONSET | self.show_function)?;
        delay.delay_ms(5);

        self.command(LCD_FUNCTIONSET | self.show_function)?;

        self.set_display(LcdDisplay::On)?;

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
        self.set_reg(REG_MODE2, 0x20)
    }

    /**
    Clear the display. The LCD display driver requires a 2ms delay after clearing, which
    is why this method requires a `delay` object.

    # Errors

    Returns a `Result` that will report I2C errors, if any.
    */
    pub fn clear(&mut self, delay: &mut dyn DelayMs<u16>) -> Result<(), <I as i2c::Write>::Error> {
        const LCD_CLEARDISPLAY: u8 = 0x01;

        let result = self.command(LCD_CLEARDISPLAY);
        delay.delay_ms(2);
        result
    }

    /**
    Set the position of the cursor

    # Errors

    Returns a `Result` that will report I2C errors, if any.
    */
    pub fn set_cursor_position(&mut self, x: u8, y: u8) -> Result<(), <I as i2c::Write>::Error> {
        let col = if y == 0_u8 { x | 0x80 } else { x | 0xC0 };
        self.command(col)
    }

    /**
    Control whether the display is on or off

    # Errors

    Returns a `Result` that will report I2C errors, if any.
    */
    pub fn set_display(&mut self, display: LcdDisplay) -> Result<(), <I as i2c::Write>::Error> {
        self.control.display = display;
        self.update_display_control()
    }

    /**
    Sets the visiblity of the cursor, which is a non-blinking _

    # Errors

    Returns a `Result` that will report I2C errors, if any.
    */
    pub fn set_cursor(&mut self, cursor: Cursor) -> Result<(), <I as i2c::Write>::Error> {
        self.control.cursor = cursor;
        self.update_display_control()
    }

    /**
    Turns on the blinking block cursor

    # Errors

    Returns a `Result` that will report I2C errors, if any.
    */
    pub fn set_blink(&mut self, blink: Blink) -> Result<(), <I as i2c::Write>::Error> {
        self.control.blink = blink;
        self.update_display_control()
    }

    /**
    Adds a single character to the current position. The cursor will advance
    after this call to the next column

    # Errors

    Returns a `Result` that will report I2C errors, if any.
    */
    pub fn write_char(&mut self, char: char) -> Result<(), <I as i2c::Write>::Error> {
        self.write_two(0x40, char as u8)
    }

    /**
    Adds a string to the current position. The cursor will advance
    after this call to the next column

    # Errors

    Returns a `Result` that will report I2C errors, if any.
    */
    pub fn write_str(&mut self, s: &str) -> Result<(), <I as i2c::Write>::Error> {
        for c in s.chars() {
            self.write_char(c)?;
        }

        Ok(())
    }

    /**
    Set the color of the backlight for displays that have an RGB backlight.

    # Errors

    Returns a `Result` that will report I2C errors, if any.
    */
    pub fn set_rgb(&mut self, r: u8, g: u8, b: u8) -> Result<(), <I as i2c::Write>::Error> {
        const REG_RED: u8       = 0x04;        // pwm2
        const REG_GREEN: u8     = 0x03;        // pwm1
        const REG_BLUE: u8      = 0x02;        // pwm0
    
        self.set_reg(REG_RED, r)?;
        self.set_reg(REG_GREEN, g)?;
        self.set_reg(REG_BLUE, b)
    }

    fn set_reg(&mut self, addr: u8, data: u8) -> Result<(), <I as i2c::Write>::Error> {
        self.i2c.write(self.rgb_address, &[addr, data])
    }

    // Set one of the display's control options and then send the updated set of options to the display
    fn update_display_control(&mut self) -> Result<(), <I as i2c::Write>::Error> {
        self.command(self.control.value())
    }

    // Send a command to the LCD display
    fn command(&mut self, value: u8) -> Result<(), <I as i2c::Write>::Error> {
        self.write_two(0x80, value)
    }

    // Send two bytes to the display
    fn write_two(&mut self, byte1: u8, byte2: u8) -> Result<(), <I as i2c::Write>::Error> {
        self.i2c.write(self.address, &[byte1, byte2])
    }
}
