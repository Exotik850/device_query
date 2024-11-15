//! Callback guard.

use std::sync::Arc;

/// Callback guard returned when adding a callback as an event listener. If the guard is dropped,
/// the event listener is removed.
// #[derive(Debug)]
pub struct CallbackGuard<Arg> {
    pub(crate) _callback: Arc<dyn Fn(Arg)>,
}
