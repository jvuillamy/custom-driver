use std::collections::HashMap;

use once_cell::sync::Lazy;

use super::{DriverBehaviour, DriverState, KeyDirection, PressType};

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
}

impl DriverBehaviour for MorseMode {
    fn handle_motion(&mut self, state: &mut DriverState, dir: KeyDirection) {
        use KeyDirection::*;

        match (self.morse.is_empty(), dir) {
            (true, LEFT) => state.controller.backspace_click(),
            (true, RIGHT) => state.controller.spacebar_click(),

            (false, LEFT) => {
                self.morse.clear();
                state.notification.notify_morse_empty();
            }

            (false, RIGHT) => {
                if let Some(morse_translation) = self.get_character() {
                    state.controller.add_character(morse_translation);
                    state.notification.notify_morse_empty();
                } else {
                    state.notification.notify_morse_invalid();
                }
                self.morse.clear();
            }
        }
    }

    fn handle_toggle(&mut self, state: &mut DriverState) {
        self.append(state.toggle_flag);
        state.notification.notify_morse_code(&self.morse);
    }

    fn handle_play(&mut self, state: &mut DriverState) {
        state.controller.return_click();
    }
}
