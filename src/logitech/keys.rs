#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeyDirection {
    LEFT,
    RIGHT,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeyCode {
    Direction(KeyDirection),
    PLAY,
    TOGGLE,
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum PressType {
    #[default]
    RELEASE,
    SHORT,
    LONG,
}

#[derive(Debug, Copy, Clone)]
pub struct Key {
    pub code: KeyCode,
    pub press: PressType,
}

impl TryFrom<evdev::InputEvent> for Key {
    type Error = String;

    fn try_from(e: evdev::InputEvent) -> Result<Key, Self::Error> {
        use evdev::InputEventKind as E;
        use evdev::Key as EK;

        use KeyCode::*;
        use KeyDirection::*;
        let code = match e.kind() {
            E::Key(EK::KEY_LEFT) => Direction(LEFT),
            E::Key(EK::KEY_RIGHT) => Direction(RIGHT),
            E::Key(EK::KEY_ENTER) => PLAY,
            E::Key(EK::KEY_F11) => TOGGLE,
            _ => Err("Could not match key event")?,
        };

        let press = match e.value() {
            0 => PressType::RELEASE,
            1 => PressType::SHORT,
            2 => PressType::LONG,
            _ => Err("Could not match key value")?,
        };

        Ok(Key { code, press })
    }
}
