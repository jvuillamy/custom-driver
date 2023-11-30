mod logitech;

#[tokio::main]
async fn main() {
    let mut device = logitech::device::get().expect("Could not find the Logitech device.");

    device
        .grab()
        .expect("Could not grab exclusive access to the device.");

    let mut stream = device
        .into_event_stream()
        .expect("Could not get device event stream.");

    let mut driver: logitech::Driver = Default::default();

    loop {
        match stream.next_event().await {
            Ok(event) => {
                if let Ok(e) = event.try_into() {
                    driver.process_event(&e);
                }
            }
            Err(_) => panic!("Could not pool device."),
        }
    }
}
