mod controller;
mod notification;

use super::{KeyDirection, PressType};

#[derive(Default)]
pub struct DriverState {
    pub controller: controller::VirtualMouseKeyboard,

    // Toast notifications system.
    pub notification: notification::Monitor,

    // The toggle button can act as a modifier for behaviours.
    pub toggle_flag: PressType,

    // We change mode by pressing both directions at the same time.
    pub mode_change_flag: Option<KeyDirection>,
}
