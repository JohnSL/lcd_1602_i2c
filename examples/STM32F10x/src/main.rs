// src/main.rs

#![no_std]
#![no_main]

use cortex_m_rt::entry; // The runtime

use hal::{
    delay::Delay,
    i2c::{self, BlockingI2c},
    pac,
    prelude::*,
};
use stm32f1xx_hal as hal; // STM32F1 specific functions

use lcd_1602_i2c::Lcd;

#[allow(unused_imports)]
use panic_halt; // When a panic occurs, stop the microcontroller

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let core_peripherals = cortex_m::Peripherals::take().unwrap();

    let mut rcc = peripherals.RCC.constrain();

    let mut gpiob = peripherals.GPIOB.split(&mut rcc.apb2);

    let mut flash = peripherals.FLASH.constrain();

    let clocks = rcc.cfgr.sysclk(8.mhz()).freeze(&mut flash.acr);

    let mut delay = Delay::new(core_peripherals.SYST, clocks);

    // Now setup the LCD display
    let scl = gpiob.pb10.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb11.into_alternate_open_drain(&mut gpiob.crh);

    let i2c_bus = BlockingI2c::i2c2(
        peripherals.I2C2,
        (scl, sda),
        i2c::Mode::Standard {
            frequency: 400_000.hz(),
        },
        clocks,
        &mut rcc.apb1,
        1000,
        10,
        1000,
        1000,
    );

    let mut lcd = Lcd::new(i2c_bus, LCD_ADDRESS, RGB_ADDRESS, &mut delay).unwrap();
    lcd.set_rgb(255, 255, 255).unwrap();
    lcd.print("Hello world!").unwrap();

    loop {
    }
}

// Device I2c addresses
const LCD_ADDRESS: u8 = 0x7c >> 1;
const RGB_ADDRESS: u8 = 0xc0 >> 1;
