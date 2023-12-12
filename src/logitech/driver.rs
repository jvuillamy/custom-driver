use crate::logitech::{
    behaviours::{DriverBehaviour, DriverMode},
    keys::{Key, KeyCode, KeyDirection, PressType},
    state::DriverState,
};

/** Driver to handle events from a Logitech R400 device. Allows to pass event
 *  and switch between different modes, represented by the [DriverMode]
 *  structure.
 */
pub struct Driver {
    state: DriverState,
    mode: DriverMode,

    // We change mode by pressing both directions at the same time.
    mode_change_flag: Option<KeyDirection>,
}

impl Default for Driver {
    fn default() -> Driver {
        Driver {
            state: Default::default(),
            mode: DriverMode::MouseMode(Default::default()),
            mode_change_flag: None,
        }
    }
}

impl Driver {
    pub fn process_event(&mut self, key: &Key) {
        use KeyCode as KC;
        use PressType as PT;

        match (key.code, key.press) {
            (KC::Direction(_), PT::RELEASE) => self.clear_mode_change(),
            (KC::Direction(d), PT::SHORT | PT::LONG) => {
                if !self.is_mode_change(d) {
                    // The modifier is considered enabled for SHORT and LONG presses.
                    // However, we don't want to trigger a SHORT press after it has
                    // been considered as modifier.
                    if self.state.toggle_flag == PressType::SHORT {
                        self.state.toggle_flag = PressType::LONG;
                    }

                    self.mode.handle_motion(&mut self.state, d);
                }
            }

            (KC::TOGGLE, PT::RELEASE) => {
                self.mode.handle_toggle(&mut self.state);
                self.state.toggle_flag = PressType::RELEASE;
            }
            (KC::TOGGLE, t @ (PT::SHORT | PT::LONG)) => self.state.toggle_flag = t,

            (KC::PLAY, PT::RELEASE) => (),
            (KC::PLAY, PT::SHORT) => self.mode.handle_play(&mut self.state),
            (KC::PLAY, PT::LONG) => unreachable!("Device does not have a LONG press on PLAY"),
        }
    }

    fn clear_mode_change(&mut self) {
        self.mode_change_flag = None;
    }

    fn is_mode_change(&mut self, dir: KeyDirection) -> bool {
        let mode_has_changed = self.mode_change_flag.is_some_and(|state| {
            if state != dir {
                self.switch_mode();
                true
            } else {
                false
            }
        });

        self.mode_change_flag = Some(dir);
        mode_has_changed
    }

    fn switch_mode(&mut self) {
        use DriverMode as DM;
        self.mode = match self.mode {
            DM::MouseMode(_) => DM::MorseMode(Default::default()),
            DM::MorseMode(_) => DM::PlayerMode(Default::default()),
            DM::PlayerMode(_) => DM::MouseMode(Default::default()),
        };

        self.state
            .notification
            .notify_with_icon(&self.mode.get_name(), self.mode.get_icon())
    }
}
