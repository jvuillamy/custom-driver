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

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub enum PressType {
    #[default]
    RELEASE,
    SHORT,
    LONG,
}

/// Defines a Logitech R400 event.
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

        use KeyCode as KC;
        use KeyDirection as KD;
        let code = match e.kind() {
            E::Key(EK::KEY_LEFT) => KC::Direction(KD::LEFT),
            E::Key(EK::KEY_RIGHT) => KC::Direction(KD::RIGHT),
            E::Key(EK::KEY_ENTER) => KC::PLAY,
            E::Key(EK::KEY_F11) => KC::TOGGLE,
            _ => return Err("Could not match key event".to_string()),
        };
        
        let press = match e.value() {
            0 => PressType::RELEASE,
            1 => PressType::SHORT,
            2 => PressType::LONG,
            _ => return Err("Could not match key value".to_string()),
        };

        Ok(Key { code, press })
    }
}
