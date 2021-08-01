// Flags for display on/off control
#[repr(u8)]
pub enum ControlOptions {
    DisplayOn = 0x04,
    CursorOn = 0x02,
    BlinkOn = 0x01,
}

/// Used to keep track of the current "control" state of the LCD. This allows us to
/// set and clear individual options (flags).
pub struct DisplayControl {
    control: u8,
}

impl DisplayControl {
    pub fn new() -> DisplayControl {
        DisplayControl { control: 0 }
    }

    /// Set a control flag in this struct
    pub fn set(&mut self, value: ControlOptions) -> &mut Self {
        self.control |= value as u8;
        self
    }

    /// Clear a control flag in this struct
    pub fn clear(&mut self, value: ControlOptions) -> &mut Self {
        self.control &= !(value as u8);
        self
    }

    /// Get the current value of the control flag
    pub fn value(&self) -> u8 {
        self.control
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_value_zero() {
        let control = DisplayControl::new();
        assert_eq!(0, control.value());
    }

    #[test]
    fn display_on_sets_value() {
        let mut control = DisplayControl::new();
        control.set(ControlOptions::CursorOn);
        assert_eq!(ControlOptions::CursorOn as u8, control.value());
    }

    #[test]
    fn two_options_on() {
        let mut control = DisplayControl::new();
        control.set(ControlOptions::CursorOn).set(ControlOptions::DisplayOn);
        assert_eq!(ControlOptions::CursorOn as u8 | ControlOptions::DisplayOn as u8, control.value());
    }

    #[test]
    fn two_options_on_then_clear_one() {
        let mut control = DisplayControl::new();
        control.set(ControlOptions::CursorOn).set(ControlOptions::DisplayOn);

        control.clear(ControlOptions::CursorOn);
        assert_eq!(ControlOptions::DisplayOn as u8, control.value());
    }
}