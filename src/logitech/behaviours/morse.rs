use std::collections::HashMap;

use once_cell::sync::Lazy;

use super::{DriverBehaviour, KeyDirection, PressType, SharedState};

/**
 * This mode allows to input letters and numbers using morse code on the
 * toggle button (SHORT/LONG presses).
 * The left key maps to the backspace when there is no current morse code or
 * clears the code otherwise. The right key maps to the spacebar when there is
 * no current morse code or inserts (if valid) the corresponding character
 * from the morse code. The play key maps to Enter.
 */
#[derive(Default)]
pub struct MorseMode {
    morse: String,
}

impl MorseMode {
    const DICT: Lazy<HashMap<&str, char>> = Lazy::new(|| {
        HashMap::from([
            // Letters
            (".-", 'a'),
            ("-...", 'b'),
            ("-.-.", 'c'),
            ("-..", 'd'),
            (".", 'e'),
            ("..-.", 'f'),
            ("--.", 'g'),
            ("....", 'h'),
            ("..", 'i'),
            (".---", 'j'),
            ("-.-", 'k'),
            (".-..", 'l'),
            ("--", 'm'),
            ("-.", 'n'),
            ("---", 'o'),
            (".--.", 'p'),
            ("--.-", 'q'),
            (".-.", 'r'),
            ("...", 's'),
            ("-", 't'),
            ("..-", 'u'),
            ("...-", 'v'),
            (".--", 'w'),
            ("-..-", 'x'),
            ("-.--", 'y'),
            ("--..", 'z'),
            // Numbers
            ("-----", '0'),
            (".----", '1'),
            ("..---", '2'),
            ("...--", '3'),
            ("....-", '4'),
            (".....", '5'),
            ("-....", '6'),
            ("--...", '7'),
            ("---..", '8'),
            ("----.", '9'),
        ])
    });

    fn append(&mut self, p: PressType) {
        match p {
            PressType::SHORT => self.morse.push('.'),
            PressType::LONG => self.morse.push('-'),
            _ => unreachable!("Invalid press type for Morse code"),
        }
    }

    fn get_character(&self) -> Option<char> {
        MorseMode::DICT.get(&self.morse as &str).copied()
    }

    fn as_text(&self) -> String {
        format!("Morse: {}", self.morse)
    }
}

impl DriverBehaviour for MorseMode {
    fn handle_motion(&mut self, state: &mut impl SharedState, dir: KeyDirection) {
        use KeyDirection as KD;
        match (self.morse.is_empty(), dir) {
            (true, KD::LEFT) => state.get_controller().backspace_click(),
            (true, KD::RIGHT) => state.get_controller().spacebar_click(),

            (false, KD::LEFT) => {
                self.morse.clear();
                state.get_notification().notify(&self.as_text());
            }

            (false, KD::RIGHT) => {
                if let Some(morse_translation) = self.get_character() {
                    state.get_controller().add_character(morse_translation);
                    state.get_notification().notify(&self.as_text());
                } else {
                    state.get_notification().notify_invalid(&self.as_text());
                }
                self.morse.clear();
            }
        }
    }

    fn handle_toggle(&mut self, state: &mut impl SharedState) {
        self.append(state.get_toggle_flag());
        state.get_notification().notify(&self.as_text());
    }

    fn handle_play(&mut self, state: &mut impl SharedState) {
        state.get_controller().return_click();
    }
}
