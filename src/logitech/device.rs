/// Gets the Logitech USB Receiver device if available.
pub fn get() -> Option<evdev::Device> {
    const DEVICE_NAME: &'static str = "Logitech USB Receiver";
    for (path, device) in evdev::enumerate() {
        if device.name().is_some_and(|n| n == DEVICE_NAME) {
            if let Ok(device) = evdev::Device::open(path) {
                return Some(device);
            }
        }
    }
    None
}
