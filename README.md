# I2C Character LCD Driver

![Screen](images/IMG_2554.jpg)

Provides an embedded Rust driver for common 16x2 LCD displays that use the AiP31068L chip to
drive the display, and a PCA9633 chip to drive the RGB backlight.

This has been tested with the [Waveshare LCD1602 module](https://www.waveshare.com/wiki/LCD1602_RGB_Module).
It may also work with other RGB displays like the [Groove 16X2 LDC RGB](https://www.seeedstudio.com/Grove-LCD-RGB-Backlight-p-1643.html),
but I haven't tested it.

This is a basic implementation, and doesn't currently support custom characters.
### License

This project is licensed under MIT license ([LICENSE](https://github.com/kunerd/clerk/blob/master/docs/CONTRIBUTING.md) or <https://opensource.org/licenses/MIT>)