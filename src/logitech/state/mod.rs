mod controller;
mod notification;

use crate::logitech::{keys::PressType, utils};

/*
    Full state of the driver.
*/
pub struct DriverState {
    // Controller to emit mouse and keyboard events.
    pub controller: controller::VirtualMouseKeyboard,

    /// Toast notifications system.
    pub notification: notification::Monitor,

    /// The toggle button can act as a modifier for behaviours.
    pub toggle_flag: PressType,
}

/// Defines an state interface accessible from the different behaviours.
pub trait SharedState {
    fn get_controller(&mut self) -> &mut controller::VirtualMouseKeyboard;
    fn get_notification(&mut self) -> &mut notification::Monitor;
    fn get_toggle_flag(&self) -> PressType;
}

impl SharedState for DriverState {
    fn get_controller(&mut self) -> &mut controller::VirtualMouseKeyboard {
        &mut self.controller
    }

    fn get_notification(&mut self) -> &mut notification::Monitor {
        &mut self.notification
    }

    fn get_toggle_flag(&self) -> PressType {
        self.toggle_flag
    }
}

impl Default for DriverState {
    fn default() -> Self {
        Self {
            controller: Default::default(),
            notification: notification::Monitor::new(
                "Welcome to the Logitech driver",
                utils::get_icon_path("welcome"),
            ),
            toggle_flag: Default::default(),
        }
    }
}
