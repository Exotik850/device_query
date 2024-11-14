//! Mouse callback.

use crate::device_events::utils;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex, Weak};
use crate::{MouseButton, MousePosition};

/// Mouse move callback.
pub trait MouseMoveCallback: Fn(MousePosition) + Sync + Send + 'static {}
impl<F: Fn(MousePosition) + Sync + Send + 'static> MouseMoveCallback for F {}

/// Mouse button callback.

pub trait MouseButtonCallback: Fn(MouseButton) + Sync + Send + 'static {}
impl<F: Fn(MouseButton) + Sync + Send + 'static> MouseButtonCallback for F {}

/// Mouse callbacks.
#[derive(Default)]
pub(crate) struct MouseCallbacks {
    pub mouse_move: Mutex<Vec<Weak<dyn MouseMoveCallback>>>,
    pub mouse_up: Mutex<Vec<Weak<dyn MouseButtonCallback>>>,
    pub mouse_down: Mutex<Vec<Weak<dyn MouseButtonCallback>>>,
}

impl MouseCallbacks {
    pub fn push_mouse_move(&self, callback: &Arc<impl MouseMoveCallback>) {
        if let Ok(mut callbacks) = self.mouse_move.lock() {
            let callback = Arc::downgrade(callback);
            callbacks.push(callback)
        }
    }

    pub fn push_mouse_down(&self, callback: &Arc<impl MouseButtonCallback>) {
        if let Ok(mut callbacks) = self.mouse_down.lock() {
            let callback = Arc::downgrade(callback);
            callbacks.push(callback)
        }
    }

    pub fn push_mouse_up(&self, callback: &Arc<impl MouseButtonCallback>) {
        if let Ok(mut callbacks) = self.mouse_up.lock() {
            let callback = Arc::downgrade(callback);
            callbacks.push(callback)
        }
    }

    pub fn run_mouse_move(&self, position: MousePosition) {
        if let Ok(mut callbacks) = self.mouse_move.lock() {
            utils::DrainFilter::drain_filter(callbacks.deref_mut(), |callback| {
                callback.upgrade().is_none()
            });
            for callback in callbacks.iter() {
                if let Some(callback) = callback.upgrade() {
                    callback(position);
                }
            }
        }
    }

    pub fn run_mouse_down(&self, button: MouseButton) {
        if let Ok(mut callbacks) = self.mouse_down.lock() {
            utils::DrainFilter::drain_filter(callbacks.deref_mut(), |callback| {
                callback.upgrade().is_none()
            });
            for callback in callbacks.iter() {
                if let Some(callback) = callback.upgrade() {
                    callback(button);
                }
            }
        }
    }

    pub fn run_mouse_up(&self, button: MouseButton) {
        if let Ok(mut callbacks) = self.mouse_up.lock() {
            utils::DrainFilter::drain_filter(callbacks.deref_mut(), |callback| {
                callback.upgrade().is_none()
            });
            for callback in callbacks.iter() {
                if let Some(callback) = callback.upgrade() {
                    callback(button);
                }
            }
        }
    }
}
