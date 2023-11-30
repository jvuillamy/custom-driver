use std::fmt::Formatter;

use enum_dispatch::enum_dispatch;

use super::{DriverState, KeyDirection, PressType};

mod morse;
mod mouse;
mod player;
use morse::MorseMode;
use mouse::MouseMode;
use player::PlayerMode;

#[enum_dispatch]
pub trait DriverBehaviour {
    fn handle_motion(&mut self, driver: &mut DriverState, dir: KeyDirection);
    fn handle_toggle(&mut self, driver: &mut DriverState);
    fn handle_play(&mut self, driver: &mut DriverState);
}

// This enum's underlying type represents the state of any mode, each of which
// implements the DriverBehaviour trait. This is very similar to a dynamic 
// dispatch but with value semantics (no Box<dyn ...> types required).
#[enum_dispatch(DriverBehaviour)]
pub enum DriverMode {
    MouseMode,
    MorseMode,
    PlayerMode,
}

impl std::fmt::Display for DriverMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            DriverMode::MouseMode(_) => write!(f, "Mouse"),
            DriverMode::MorseMode(_) => write!(f, "Morse"),
            DriverMode::PlayerMode(_) => write!(f, "Player"),
        }
    }
}
