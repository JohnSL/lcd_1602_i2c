#[derive(Copy, Clone)]
pub enum Cursor {
    On = 2,
    Off = 0,
}

#[derive(Copy, Clone)]
pub enum LcdDisplay {
    On = 4,
    Off = 0,
}

#[derive(Copy, Clone)]
pub enum Blink {
    On = 1,
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
