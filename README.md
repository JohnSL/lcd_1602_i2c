# I2C Character LCD Driver

![Screen](images/IMG_2554.jpg)

Provides an embedded Rust driver for common 16x2 LCD displays that use the AiP31068L chip to
drive the display, and a PCA9633 chip to drive the RGB backlight.

This has been tested with the [Waveshare LCD1602 module](https://www.waveshare.com/wiki/LCD1602_RGB_Module).
It may also work with other RGB displays like the [Groove 16X2 LDC RGB](https://www.seeedstudio.com/Grove-LCD-RGB-Backlight-p-1643.html),
but I haven't tested it.

This is a basic implementation, and doesn't currently support custom characters.

## Speed

This driver is fast enough that there are no noticable delays when updating text on the screen. In
my programs, I'm using spaces to clear the ends of lines rather than clearing the screen.

## Example

Currently there is a single, but working, example of using this crate in the [examples/STM32F10x](examples/STM32F10x) folder.
The example is using an STM32F103RB Nucleo-64 board. This is a nice little board as it has the programmer/debugger
built into the board.

```rust
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
```

## License

This project is licensed under MIT license ([LICENSE](https://github.com/kunerd/clerk/blob/master/docs/CONTRIBUTING.md) or <https://opensource.org/licenses/MIT>)