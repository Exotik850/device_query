use super::{CallbackGuard, KeyboardCallbacks};
use std::sync::{Arc, Mutex, Weak};
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;
use MouseState;
use {DeviceQuery, MouseCallbacks};
use {DeviceState, Keycode};
use {MouseButton, MousePosition};

pub(crate) struct EventLoop {
    keyboard_callbacks: Arc<KeyboardCallbacks>,
    mouse_callbacks: Arc<MouseCallbacks>,
    _keyboard_thread: JoinHandle<()>,
    _mouse_thread: JoinHandle<()>,
}

fn keyboard_thread(callbacks: Weak<KeyboardCallbacks>, sleep_dur: Duration) -> JoinHandle<()> {
    spawn(move || {
        let device_state = DeviceState::new();
        let mut prev_keys = vec![];
        while let Some(callbacks) = callbacks.upgrade() {
            let keys = device_state.get_keys();
            for key_state in keys.iter().copied() {
                if !prev_keys.contains(&key_state) {
                    callbacks.run_key_down(key_state);
                }
            }
            for key_state in prev_keys.drain(..) {
                if !keys.contains(&key_state) {
                    callbacks.run_key_up(key_state);
                }
            }
            prev_keys = keys;
            sleep(sleep_dur);
        }
    })
}

fn mouse_thread(callbacks: Weak<MouseCallbacks>, sleep_dur: Duration) -> JoinHandle<()> {
    spawn(move || {
        let device_state = DeviceState::new();
        let mut previous_mouse_state = MouseState::default();
        while let Some(callbacks) = callbacks.upgrade() {
            let mouse_state = device_state.get_mouse();
            for (index, (previous_state, current_state)) in previous_mouse_state
                .button_pressed
                .iter()
                .zip(mouse_state.button_pressed.iter())
                .enumerate()
            {
                if !(*previous_state) && *current_state {
                    callbacks.run_mouse_down(index);
                } else if *previous_state && !(*current_state) {
                    callbacks.run_mouse_up(index);
                }
            }
            if mouse_state.coords != previous_mouse_state.coords {
                callbacks.run_mouse_move(mouse_state.coords);
            }
            previous_mouse_state = mouse_state;
            sleep(sleep_dur);
        }
    })
}

impl Default for EventLoop {
    fn default() -> Self {
        Self::new(Duration::from_micros(100))
    }
}

impl EventLoop {
    fn new(sleep_dur: Duration) -> Self {
        let keyboard_callbacks = Arc::new(KeyboardCallbacks::default());
        let mouse_callbacks = Arc::new(MouseCallbacks::default());
        let _keyboard_thread = keyboard_thread(Arc::downgrade(&keyboard_callbacks), sleep_dur);
        let _mouse_thread = mouse_thread(Arc::downgrade(&mouse_callbacks), sleep_dur);
        Self {
            keyboard_callbacks,
            mouse_callbacks,
            _keyboard_thread,
            _mouse_thread,
        }
    }

    pub fn on_key_down<Callback: Fn(Keycode) + Send + Sync + 'static>(
        &mut self,
        callback: Callback,
    ) -> CallbackGuard<Keycode> {
        let _callback = Arc::new(callback);
        self.keyboard_callbacks.push_key_down(&_callback);
        CallbackGuard { _callback }
    }

    pub fn on_key_up<Callback: Fn(Keycode) + Send + Sync + 'static>(
        &mut self,
        callback: Callback,
    ) -> CallbackGuard<Keycode> {
        let _callback = Arc::new(callback);
        self.keyboard_callbacks.push_key_up(&_callback);
        CallbackGuard { _callback }
    }

    pub fn on_mouse_move<Callback: Fn(MousePosition) + Send + Sync + 'static>(
        &mut self,
        callback: Callback,
    ) -> CallbackGuard<MousePosition> {
        let _callback = Arc::new(callback);
        self.mouse_callbacks.push_mouse_move(&_callback);
        CallbackGuard { _callback }
    }

    pub fn on_mouse_up<Callback: Fn(MouseButton) + Send + Sync + 'static>(
        &mut self,
        callback: Callback,
    ) -> CallbackGuard<MouseButton> {
        let _callback = Arc::new(callback);
        self.mouse_callbacks.push_mouse_up(&_callback);
        CallbackGuard { _callback }
    }

    pub fn on_mouse_down<Callback: Fn(MouseButton) + Send + Sync + 'static>(
        &mut self,
        callback: Callback,
    ) -> CallbackGuard<MouseButton> {
        let _callback = Arc::new(callback);
        self.mouse_callbacks.push_mouse_down(&_callback);
        CallbackGuard { _callback }
    }
}

lazy_static! {
    pub(crate) static ref EVENT_LOOP: Arc<Mutex<Option<EventLoop>>> = Default::default();
}

pub fn init_event_loop(sleep_dur: Duration) -> bool {
    let mut event_loop = EVENT_LOOP.lock().unwrap();
    if event_loop.is_none() {
        *event_loop = Some(EventLoop::new(sleep_dur));
        true
    } else {
        false
    }
}