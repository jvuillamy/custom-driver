use notify_rust::{Notification, NotificationHandle};

pub struct Monitor {
    handle: NotificationHandle,
    icon: String,
}

impl Monitor {
    pub fn new(message: &str, icon: String) -> Self {
        Monitor {
            handle: Notification::new()
                .summary(message)
                .icon(&icon)
                .show()
                .unwrap(),
            icon,
        }
    }

    // Updates with a new message and icon.
    pub fn notify_with_icon(&mut self, summary: &str, icon: String) {
        self.icon = icon;
        self.notify(summary);
    }

    /// Updates with a new message using the icon used before any calls
    /// to [Self::notify_invalid].
    pub fn notify(&mut self, summary: &str) {
        self.handle.summary(summary).icon(&self.icon);
        self.handle.update();
    }

    // Updates with a new message using an error icon.
    pub fn notify_invalid(&mut self, summary: &str) {
        self.handle.summary(summary).icon("error");
        self.handle.update();
    }
}
