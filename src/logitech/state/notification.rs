use notify_rust::{Notification, NotificationHandle};

use crate::logitech::DriverMode;

fn icon_path(icon_name: &str) -> String {
    std::fs::canonicalize(format!("./icons/{icon_name}.png"))
        .unwrap_or_default()
        .to_str()
        .unwrap()
        .into()
}

fn mode_as_icon(mode: &DriverMode) -> String {
    match mode {
        DriverMode::MouseMode(_) => icon_path("mouse"),
        DriverMode::MorseMode(_) => icon_path("antenna"),
        DriverMode::PlayerMode(_) => icon_path("movie"),
    }
}

pub struct Monitor {
    handle: NotificationHandle,
}

impl Default for Monitor {
    fn default() -> Self {
        Self {
            handle: Notification::new()
                .summary("Starting driver...")
                .icon(icon_path("welcome").as_str())
                .show()
                .unwrap(),
        }
    }
}

impl Monitor {
    pub fn notify_mode_change(&mut self, mode: &DriverMode) {
        self.handle
            .summary(&format!("Mode changed: {}", mode.to_string()) as &str)
            .icon(mode_as_icon(mode).as_str());
        self.handle.update();
    }

    pub fn notify_morse_empty(&mut self) {
        self.handle.summary("Current code: ");
        self.handle.update();
    }
    
    pub fn notify_morse_code(&mut self, code: &String) {
        self.handle
            .summary(&format!("Current code: {code}") as &str);
        self.handle.update();
    }

    pub fn notify_morse_invalid(&mut self) {
        self.handle.summary("Invalid morse code !").icon("error");
        self.handle.update();
    }
}
