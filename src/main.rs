mod logitech;

#[tokio::main]
async fn main() -> Result<(), String> {
    let mut device = match logitech::device::get() {
        Some(d) => d,
        None => return Err("Could not find a Logitech device.".to_string()),
    };

    if let Err(e) = device.grab() {
        return Err(format!("Invalid device access: {e}"));
    }

    let mut stream = device
        .into_event_stream()
        .map_err(|e| format!("Could not get event stream: {e}"))?;

    let mut driver: logitech::Driver = Default::default();

    loop {
        match stream.next_event().await {
            Ok(event) => {
                if let Ok(e) = event.try_into() {
                    driver.process_event(&e);
                }
            }
            Err(_) => return Err("Could not pool device (maybe unplugged ?)".to_string()),
        }
    }
}
