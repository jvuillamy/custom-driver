use enum_dispatch::enum_dispatch;

use crate::logitech::{
    keys::{KeyDirection, PressType},
    state::SharedState,
    utils,
};

mod morse;
mod mouse;
mod player;
use morse::MorseMode;
use mouse::MouseMode;
use player::PlayerMode;

#[enum_dispatch]
pub trait DriverBehaviour {
    fn handle_motion(&mut self, state: &mut impl SharedState, dir: KeyDirection);
    fn handle_toggle(&mut self, state: &mut impl SharedState);
    fn handle_play(&mut self, state: &mut impl SharedState);
}

/// This enum's underlying type represents the state of any mode, each of which
/// implements the DriverBehaviour trait. This is very similar to a dynamic
/// dispatch but with value semantics (no Box<dyn ...> types required).
#[enum_dispatch(DriverBehaviour)]
pub enum DriverMode {
    MouseMode,
    MorseMode,
    PlayerMode,
}

impl DriverMode {
    pub fn get_name(&self) -> String {
        use DriverMode as DM;
        match self {
            DM::MouseMode(_) => "Mode: Mouse".to_string(),
            DM::MorseMode(_) => "Mode: Morse".to_string(),
            DM::PlayerMode(_) => "Mode: Player".to_string(),
        }
    }

    pub fn get_icon(&self) -> String {
        match self {
            DriverMode::MouseMode(_) => utils::get_icon_path("mouse"),
            DriverMode::MorseMode(_) => utils::get_icon_path("antenna"),
            DriverMode::PlayerMode(_) => utils::get_icon_path("movie"),
        }
    }
}
