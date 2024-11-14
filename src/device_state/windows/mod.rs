extern crate windows;

use keyboard_types::Code;
use crate::MouseState;
use self::windows::Win32::Foundation::POINT;
use self::windows::Win32::UI::Input::KeyboardAndMouse;
use self::windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VIRTUAL_KEY};
use self::windows::Win32::UI::WindowsAndMessaging::GetCursorPos;


#[derive(Debug, Clone)]
pub struct DeviceState;

impl DeviceState {
    pub fn new() -> Self {
        Self {}
    }

    // Adding because Linux and OSX supports this where `new` can panic.
    pub fn checked_new() -> Option<Self> {
        Some(Self::new())
    }

    pub fn query_pointer(&self) -> MouseState {
        let point = &mut POINT { x: 0, y: 0 };
        let button1pressed;
        let button2pressed;
        let button3pressed;
        let button4pressed;
        let button5pressed;
        let coords;
        unsafe {
            coords = if GetCursorPos(point).is_ok() {
                (point.x, point.y)
            } else {
                (0, 0)
            };
            button1pressed =
                GetAsyncKeyState(KeyboardAndMouse::VK_LBUTTON.0 as i32) as u32 & 0x8000 != 0;
            button2pressed =
                GetAsyncKeyState(KeyboardAndMouse::VK_RBUTTON.0 as i32) as u32 & 0x8000 != 0;
            button3pressed =
                GetAsyncKeyState(KeyboardAndMouse::VK_MBUTTON.0 as i32) as u32 & 0x8000 != 0;
            button4pressed =
                GetAsyncKeyState(KeyboardAndMouse::VK_XBUTTON1.0 as i32) as u32 & 0x8000 != 0;
            button5pressed =
                GetAsyncKeyState(KeyboardAndMouse::VK_XBUTTON2.0 as i32) as u32 & 0x8000 != 0;
        }
        MouseState {
            coords,
            // button_pressed: vec![
            //     false,
            //     button1pressed,
            //     button2pressed,
            //     button3pressed,
            //     button4pressed,
            //     button5pressed,
            // ],
            button_pressed: [
                button1pressed,
                button2pressed,
                button3pressed,
                button4pressed,
                button5pressed,
            ],
        }
    }

    pub fn query_keymap(&self) -> Vec<Code> {
        let mut keycodes = vec![];
        let mut keymap = vec![];
        unsafe {
            for key in 0..256 {
                keymap.push(GetAsyncKeyState(key));
            }
        }
        for (ix, byte) in keymap.iter().enumerate() {
            if *byte as u32 & 0x8000 != 0 {
                if let Some(k) = self.win_key_to_keycode(ix as u16) {
                    keycodes.push(k)
                }
            }
        }
        keycodes
    }

    fn win_key_to_keycode(&self, win_key: u16) -> Option<Code> {
        let mut keycode = match VIRTUAL_KEY(win_key) {
            KeyboardAndMouse::VK_F1 => Some(Code::F1),
            KeyboardAndMouse::VK_F2 => Some(Code::F2),
            KeyboardAndMouse::VK_F3 => Some(Code::F3),
            KeyboardAndMouse::VK_F4 => Some(Code::F4),
            KeyboardAndMouse::VK_F5 => Some(Code::F5),
            KeyboardAndMouse::VK_F6 => Some(Code::F6),
            KeyboardAndMouse::VK_F7 => Some(Code::F7),
            KeyboardAndMouse::VK_F8 => Some(Code::F8),
            KeyboardAndMouse::VK_F9 => Some(Code::F9),
            KeyboardAndMouse::VK_F10 => Some(Code::F10),
            KeyboardAndMouse::VK_F11 => Some(Code::F11),
            KeyboardAndMouse::VK_F12 => Some(Code::F12),
            KeyboardAndMouse::VK_F13 => Some(Code::F13),
            KeyboardAndMouse::VK_F14 => Some(Code::F14),
            KeyboardAndMouse::VK_F15 => Some(Code::F15),
            KeyboardAndMouse::VK_F16 => Some(Code::F16),
            KeyboardAndMouse::VK_F17 => Some(Code::F17),
            KeyboardAndMouse::VK_F18 => Some(Code::F18),
            KeyboardAndMouse::VK_F19 => Some(Code::F19),
            KeyboardAndMouse::VK_F20 => Some(Code::F20),
            KeyboardAndMouse::VK_NUMPAD0 => Some(Code::Numpad0),
            KeyboardAndMouse::VK_NUMPAD1 => Some(Code::Numpad1),
            KeyboardAndMouse::VK_NUMPAD2 => Some(Code::Numpad2),
            KeyboardAndMouse::VK_NUMPAD3 => Some(Code::Numpad3),
            KeyboardAndMouse::VK_NUMPAD4 => Some(Code::Numpad4),
            KeyboardAndMouse::VK_NUMPAD5 => Some(Code::Numpad5),
            KeyboardAndMouse::VK_NUMPAD6 => Some(Code::Numpad6),
            KeyboardAndMouse::VK_NUMPAD7 => Some(Code::Numpad7),
            KeyboardAndMouse::VK_NUMPAD8 => Some(Code::Numpad8),
            KeyboardAndMouse::VK_NUMPAD9 => Some(Code::Numpad9),
            KeyboardAndMouse::VK_ADD => Some(Code::NumpadAdd),
            KeyboardAndMouse::VK_SUBTRACT => Some(Code::NumpadSubtract),
            KeyboardAndMouse::VK_DIVIDE => Some(Code::NumpadDivide),
            KeyboardAndMouse::VK_MULTIPLY => Some(Code::NumpadMultiply),
            KeyboardAndMouse::VK_OEM_NEC_EQUAL => Some(Code::NumpadEqual),
            KeyboardAndMouse::VK_DECIMAL => Some(Code::NumpadDecimal),
            KeyboardAndMouse::VK_SPACE => Some(Code::Space),
            KeyboardAndMouse::VK_LCONTROL => Some(Code::ControlLeft),
            KeyboardAndMouse::VK_RCONTROL => Some(Code::ControlRight),
            KeyboardAndMouse::VK_LSHIFT => Some(Code::ShiftLeft),
            KeyboardAndMouse::VK_RSHIFT => Some(Code::ShiftRight),
            KeyboardAndMouse::VK_LMENU => Some(Code::AltLeft),
            KeyboardAndMouse::VK_RMENU => Some(Code::AltRight),
            KeyboardAndMouse::VK_LWIN => Some(Code::MetaLeft),
            KeyboardAndMouse::VK_RWIN => Some(Code::MetaRight),
            KeyboardAndMouse::VK_RETURN => Some(Code::Enter),
            KeyboardAndMouse::VK_ESCAPE => Some(Code::Escape),
            KeyboardAndMouse::VK_UP => Some(Code::ArrowUp),
            KeyboardAndMouse::VK_DOWN => Some(Code::ArrowDown),
            KeyboardAndMouse::VK_LEFT => Some(Code::ArrowLeft),
            KeyboardAndMouse::VK_RIGHT => Some(Code::ArrowRight),
            KeyboardAndMouse::VK_BACK => Some(Code::Backspace),
            KeyboardAndMouse::VK_CAPITAL => Some(Code::CapsLock),
            KeyboardAndMouse::VK_TAB => Some(Code::Tab),
            KeyboardAndMouse::VK_HOME => Some(Code::Home),
            KeyboardAndMouse::VK_END => Some(Code::End),
            KeyboardAndMouse::VK_PRIOR => Some(Code::PageUp),
            KeyboardAndMouse::VK_NEXT => Some(Code::PageDown),
            KeyboardAndMouse::VK_INSERT => Some(Code::Insert),
            KeyboardAndMouse::VK_DELETE => Some(Code::Delete),
            KeyboardAndMouse::VK_OEM_3 => Some(Code::Backquote),
            KeyboardAndMouse::VK_OEM_MINUS => Some(Code::Minus),
            KeyboardAndMouse::VK_OEM_PLUS => Some(Code::Equal),
            KeyboardAndMouse::VK_OEM_4 => Some(Code::BracketLeft),
            KeyboardAndMouse::VK_OEM_6 => Some(Code::BracketRight),
            KeyboardAndMouse::VK_OEM_5 => Some(Code::Backslash),
            KeyboardAndMouse::VK_OEM_1 => Some(Code::Semicolon),
            KeyboardAndMouse::VK_OEM_7 => Some(Code::Quote),
            KeyboardAndMouse::VK_OEM_COMMA => Some(Code::Comma),
            KeyboardAndMouse::VK_OEM_PERIOD => Some(Code::Period),
            KeyboardAndMouse::VK_OEM_2 => Some(Code::Slash),

            _ => None,
        };

        if keycode.is_none() {
            let win_key = win_key as u8;
            keycode = match win_key as char {
                '0' => Some(Code::Digit0),
                '1' => Some(Code::Digit1),
                '2' => Some(Code::Digit2),
                '3' => Some(Code::Digit3),
                '4' => Some(Code::Digit4),
                '5' => Some(Code::Digit5),
                '6' => Some(Code::Digit6),
                '7' => Some(Code::Digit7),
                '8' => Some(Code::Digit8),
                '9' => Some(Code::Digit9),
                'A' => Some(Code::KeyA),
                'B' => Some(Code::KeyB),
                'C' => Some(Code::KeyC),
                'D' => Some(Code::KeyD),
                'E' => Some(Code::KeyE),
                'F' => Some(Code::KeyF),
                'G' => Some(Code::KeyG),
                'H' => Some(Code::KeyH),
                'I' => Some(Code::KeyI),
                'J' => Some(Code::KeyJ),
                'K' => Some(Code::KeyK),
                'L' => Some(Code::KeyL),
                'M' => Some(Code::KeyM),
                'N' => Some(Code::KeyN),
                'O' => Some(Code::KeyO),
                'P' => Some(Code::KeyP),
                'Q' => Some(Code::KeyQ),
                'R' => Some(Code::KeyR),
                'S' => Some(Code::KeyS),
                'T' => Some(Code::KeyT),
                'U' => Some(Code::KeyU),
                'V' => Some(Code::KeyV),
                'W' => Some(Code::KeyW),
                'X' => Some(Code::KeyX),
                'Y' => Some(Code::KeyY),
                'Z' => Some(Code::KeyZ),
                _ => None,
            }
        }
        keycode
    }
}
