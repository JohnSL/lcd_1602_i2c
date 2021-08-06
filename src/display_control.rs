#![deny(missing_docs)]

/// Controls the visibilty of the non-blinking cursor, which is basically an _ **after** the cursor position.
/// The cursor position represents where the next character will show up.
#[derive(Copy, Clone)]
pub enum Cursor {
    /// Display the non-blinking cursor
    On = 2,
    /// Hide the non-blinking cursor
    Off = 0,
}

/// Determines whether the entire LCD is on or off.
#[derive(Copy, Clone)]
pub enum LcdDisplay {
    /// Turn the LCD display on
    On = 4,
    /// Turn the LCD display off
    Off = 0,
}

/// Controls the visibility of the blinking block cursor.
#[derive(Copy, Clone)]
pub enum Blink {
    /// Turn the blinking block cursor on
    On = 1,
    /// Turn the blinking block cursor off
    Off = 0,
}

pub struct DisplayControl {
    pub cursor: Cursor,
    pub display: LcdDisplay,
    pub blink: Blink,
}

impl DisplayControl {
    pub fn new() -> Self {
        DisplayControl {
            cursor: Cursor::Off,
            display: LcdDisplay::Off,
            blink: Blink::Off,
        }
    }
    pub fn value(&self) -> u8 {
        0x08 | self.blink as u8 | self.cursor as u8 | self.display as u8
    }
}

#[cfg(test)]
mod test2 {
    use super::*;

    #[test]
    fn all_flags_off() {
        let control = DisplayControl::new();
        assert_eq!(0x08, control.value());
    }

    #[test]
    fn cursur_on() {
        let mut control = DisplayControl::new();
        control.cursor = Cursor::On;
        assert_eq!(0x08 | 2, control.value());
    }

    #[test]
    fn blink_on() {
        let mut control = DisplayControl::new();
        control.blink = Blink::On;
        assert_eq!(0x08 | 1, control.value());
    }

    #[test]
    fn display_on() {
        let mut control = DisplayControl::new();
        control.display = LcdDisplay::On;
        assert_eq!(0x08 | 4, control.value());
    }

    #[test]
    fn display_and_cursor_on() {
        let mut control = DisplayControl::new();
        control.display = LcdDisplay::On;
        control.cursor = Cursor::On;
        assert_eq!(0x08 | 4 | 2, control.value());
    }
}
