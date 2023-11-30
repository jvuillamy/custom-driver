pub mod device;
pub mod keys;

mod behaviours;
mod state;

use behaviours::{DriverBehaviour, DriverMode};
use keys::{Key, KeyCode, KeyDirection, PressType};
use state::DriverState;

pub struct Driver {
    state: DriverState,
    mode: DriverMode,
}

impl Default for Driver {
    fn default() -> Driver {
        Driver {
            state: Default::default(),
            mode: DriverMode::MouseMode(Default::default()),
        }
    }
}

impl Driver {
    pub fn process_event(&mut self, key: &Key) {
        use KeyCode::*;
        use PressType::*;

        match (key.code, key.press) {
            (Direction(_), RELEASE) => self.clear_mode_change(),
            (Direction(d), SHORT | LONG) => {
                if !self.is_mode_change(d) {
                    // We don't want to trigger a SHORT press after
                    // the motion handling considers it a LONG press.
                    if self.state.toggle_flag == PressType::SHORT {
                        self.state.toggle_flag = PressType::LONG;
                    }

                    self.mode.handle_motion(&mut self.state, d);
                }
            }

            (TOGGLE, RELEASE) => {
                self.mode.handle_toggle(&mut self.state);
                self.state.toggle_flag = PressType::RELEASE;
            }
            (TOGGLE, t @ (SHORT | LONG)) => self.state.toggle_flag = t,

            (PLAY, RELEASE) => (),
            (PLAY, SHORT) => self.mode.handle_play(&mut self.state),
            (PLAY, LONG) => unreachable!("Device does not have a LONG press on PLAY"),
        }
    }

    fn clear_mode_change(&mut self) {
        self.state.mode_change_flag = None;
    }

    fn is_mode_change(&mut self, dir: KeyDirection) -> bool {
        let mode_has_changed = self.state.mode_change_flag.is_some_and(|state| {
            if state != dir {
                self.switch_mode();
                true
            } else {
                false
            }
        });

        self.state.mode_change_flag = Some(dir);
        mode_has_changed
    }

    fn switch_mode(&mut self) {
        use DriverMode::*;
        self.mode = match self.mode {
            MouseMode(_) => MorseMode(Default::default()),
            MorseMode(_) => PlayerMode(Default::default()),
            PlayerMode(_) => MouseMode(Default::default()),
        };

        self.state.notification.notify_mode_change(&self.mode)
    }
}
