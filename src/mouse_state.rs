//! Description of mouse coordinates and state of buttons.

/// Mouse position.
pub type MousePosition = (i32, i32);

/// MouseButton.
pub type MouseButton = usize;

#[derive(Debug, PartialEq, Default, Clone)]
/// A simple structure containing the current mouse coordinates and the
/// state of each mouse button that we can query. Currently, Windows and
/// Linux provide nice ways to query five mouse buttons.
pub struct MouseState {
    /// Coordinates in pixel.
    pub coords: MousePosition,
    /// State of each mouse button.
    pub button_pressed: [bool; 5],
}
