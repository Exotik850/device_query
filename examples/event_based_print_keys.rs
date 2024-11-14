extern crate device_query;

use device_query::{DeviceEvents, DeviceEventsHandler};
use std::thread;
use std::time::Duration;

fn main() {
    let device_state =
        DeviceEventsHandler::new(Duration::from_millis(10)).expect("Couldn't start event loop");
    let _guard = device_state.on_key_down(|key| {
        println!("Down: {:#?}", key);
    });
    let _guard = device_state.on_key_up(|key| {
        println!("Up: {:#?}", key);
    });

    loop {
        thread::sleep(Duration::from_secs(1000));
    }
}
