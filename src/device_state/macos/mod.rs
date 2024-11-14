extern crate macos_accessibility_client;

use keymap::Code;
use mouse_state::MouseState;

#[derive(Debug, Clone)]
pub struct DeviceState;
const MAPPING: &[(readkey::Code, Code)] = &[
    (readkey::Code::_0, Code::Digit0),
    (readkey::Code::_1, Code::Digit1),
    (readkey::Code::_2, Code::Digit2),
    (readkey::Code::_3, Code::Digit3),
    (readkey::Code::_4, Code::Digit4),
    (readkey::Code::_5, Code::Digit5),
    (readkey::Code::_6, Code::Digit6),
    (readkey::Code::_7, Code::Digit7),
    (readkey::Code::_8, Code::Digit8),
    (readkey::Code::_9, Code::Digit9),
    (readkey::Code::A, Code::KeyA),
    (readkey::Code::B, Code::KeyB),
    (readkey::Code::C, Code::KeyC),
    (readkey::Code::D, Code::KeyD),
    (readkey::Code::E, Code::KeyE),
    (readkey::Code::F, Code::KeyF),
    (readkey::Code::G, Code::KeyG),
    (readkey::Code::H, Code::KeyH),
    (readkey::Code::I, Code::KeyI),
    (readkey::Code::J, Code::KeyJ),
    (readkey::Code::K, Code::KeyK),
    (readkey::Code::L, Code::KeyL),
    (readkey::Code::M, Code::KeyM),
    (readkey::Code::N, Code::KeyN),
    (readkey::Code::O, Code::KeyO),
    (readkey::Code::P, Code::KeyP),
    (readkey::Code::Q, Code::KeyQ),
    (readkey::Code::R, Code::KeyR),
    (readkey::Code::S, Code::KeyS),
    (readkey::Code::T, Code::KeyT),
    (readkey::Code::U, Code::KeyU),
    (readkey::Code::V, Code::KeyV),
    (readkey::Code::W, Code::KeyW),
    (readkey::Code::X, Code::KeyX),
    (readkey::Code::Y, Code::KeyY),
    (readkey::Code::Z, Code::KeyZ),
    (readkey::Code::F1, Code::F1),
    (readkey::Code::F2, Code::F2),
    (readkey::Code::F3, Code::F3),
    (readkey::Code::F4, Code::F4),
    (readkey::Code::F5, Code::F5),
    (readkey::Code::F6, Code::F6),
    (readkey::Code::F7, Code::F7),
    (readkey::Code::F8, Code::F8),
    (readkey::Code::F9, Code::F9),
    (readkey::Code::F10, Code::F10),
    (readkey::Code::F11, Code::F11),
    (readkey::Code::F12, Code::F12),
    (readkey::Code::F13, Code::F13),
    (readkey::Code::F14, Code::F14),
    (readkey::Code::F15, Code::F15),
    (readkey::Code::F16, Code::F16),
    (readkey::Code::F17, Code::F17),
    (readkey::Code::F18, Code::F18),
    (readkey::Code::F19, Code::F19),
    (readkey::Code::F20, Code::F20),
    (readkey::Code::Keypad0, Code::Numpad0),
    (readkey::Code::Keypad1, Code::Numpad1),
    (readkey::Code::Keypad2, Code::Numpad2),
    (readkey::Code::Keypad3, Code::Numpad3),
    (readkey::Code::Keypad4, Code::Numpad4),
    (readkey::Code::Keypad5, Code::Numpad5),
    (readkey::Code::Keypad6, Code::Numpad6),
    (readkey::Code::Keypad7, Code::Numpad7),
    (readkey::Code::Keypad8, Code::Numpad8),
    (readkey::Code::Keypad9, Code::Numpad9),
    (readkey::Code::KeypadPlus, Code::NumpadAdd),
    (readkey::Code::KeypadMinus, Code::NumpadSubtract),
    (readkey::Code::KeypadDivide, Code::NumpadDivide),
    (readkey::Code::KeypadMultiply, Code::NumpadMultiply),
    (readkey::Code::KeypadEquals, Code::NumpadEquals),
    (readkey::Code::KeypadEnter, Code::NumpadEnter),
    (readkey::Code::KeypadDecimal, Code::NumpadDecimal),
    (readkey::Code::Escape, Code::Escape),
    (readkey::Code::Space, Code::Space),
    (readkey::Code::Control, Code::LControl),
    (readkey::Code::ArrowRightControl, Code::RControl),
    (readkey::Code::Shift, Code::LShift),
    (readkey::Code::ArrowRightShift, Code::ShiftRight),
    (readkey::Code::Option, Code::LOption),
    (readkey::Code::ArrowRightOption, Code::ROption),
    (readkey::Code::Command, Code::Command),
    (readkey::Code::Return, Code::Enter),
    (readkey::Code::ArrowUp, Code::ArrowUp),
    (readkey::Code::ArrowDown, Code::ArrowDown),
    (readkey::Code::ArrowLeft, Code::ArrowLeft),
    (readkey::Code::ArrowRight, Code::ArrowRight),
    (readkey::Code::Delete, Code::Backspace),
    (readkey::Code::CapsLock, Code::CapsLock),
    (readkey::Code::Tab, Code::Tab),
    (readkey::Code::Home, Code::Home),
    (readkey::Code::End, Code::End),
    (readkey::Code::PageUp, Code::PageUp),
    (readkey::Code::PageDown, Code::PageDown),
    (readkey::Code::Help, Code::Insert),
    (readkey::Code::ForwardDelete, Code::Delete),
    (readkey::Code::Backquote, Code::Backquote),
    (readkey::Code::Minus, Code::Minus),
    (readkey::Code::Equal, Code::Equal),
    (readkey::Code::BracketLeft, Code::BracketLeft),
    (readkey::Code::BracketRight, Code::BracketRight),
    (readkey::Code::Backslash, Code::Backslash),
    (readkey::Code::Semicolon, Code::Semicolon),
    (readkey::Code::Quote, Code::Quote),
    (readkey::Code::Comma, Code::Comma),
    (readkey::Code::Period, Code::Period),
    (readkey::Code::Slash, Code::Slash),
];

impl DeviceState {
    pub fn new() -> DeviceState {
        // TODO: remove this
        assert!(
            has_accessibility(),
            "This app does not have Accessibility Permissions enabled and will not work"
        );

        DeviceState {}
    }

    /// returns `None` if app doesn't accessibility permissions.
    pub fn checked_new() -> Option<DeviceState> {
        if has_accessibility() {
            Some(DeviceState {})
        } else {
            None
        }
    }

    pub fn query_pointer(&self) -> MouseState {
        let (x, y) = readmouse::Mouse::location();
        let button_pressed = [
            false,
            readmouse::Mouse::Left.is_pressed(),
            readmouse::Mouse::Right.is_pressed(),
            readmouse::Mouse::Center.is_pressed(),
            false,
        ];

        MouseState {
            coords: (x as i32, y as i32),
            button_pressed,
        }
    }

    pub fn query_keymap(&self) -> Vec<Code> {
        MAPPING
            .iter()
            .filter(|(from, _)| from.is_pressed())
            .map(|(_, to)| *to)
            .collect()
    }
}

/// Returns true if the Accessibility permissions necessary for this library to work are granted
/// to this process
///
/// If this returns false, the app can request them through the OS APIs, or the user can:
///   1. open the MacOS system preferences
///   2. go to Security -> Privacy
///   3. scroll down to Accessibility and unlock it
///   4. Add the app that is using device_query (such as your terminal) to the list
///
fn has_accessibility() -> bool {
    use self::macos_accessibility_client::accessibility::*;
    // Without prompting:
    // application_is_trusted()

    // With prompting:
    application_is_trusted_with_prompt()
}
