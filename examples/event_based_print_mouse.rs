extern crate device_query;

use device_query::{DeviceEvents, DeviceEventsHandler};
use std::thread;
use std::time::Duration;

fn main() {
    let device_state = DeviceEventsHandler::new(Duration::from_millis(10)).expect("Couldn't start event loop");
    let _guard = device_state.on_mouse_move(|position| {
        println!("Position: {:#?}", position);
    });
    let _guard = device_state.on_mouse_down(|button| {
        println!("Down: {:#?}", button);
    });
    let _guard = device_state.on_mouse_up(|button| {
        println!("Up: {:#?}", button);
    });

    loop {
        thread::sleep(Duration::from_secs(1000));
    }
}
