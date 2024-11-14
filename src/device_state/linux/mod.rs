extern crate x11;

use self::x11::xlib;
use keymap::Code;
use mouse_state::MouseState;
use std::os::raw::c_char;
use std::ptr;
use std::rc::Rc;
use std::slice;

mod kernel_key;

#[derive(Debug, Clone)]
/// Device state descriptor.
pub struct DeviceState {
    xc: Rc<X11Connection>,
}

#[derive(Debug)]
struct X11Connection {
    display: *mut xlib::Display,
}

impl Drop for X11Connection {
    fn drop(&mut self) {
        unsafe {
            xlib::XCloseDisplay(self.display);
        }
    }
}

impl DeviceState {
    /// Creates a new DeviceState.
    pub fn new() -> DeviceState {
        unsafe {
            let display = xlib::XOpenDisplay(ptr::null());
            if display.as_ref().is_none() {
                panic!("Could not connect to a X display");
            }
            DeviceState {
                xc: Rc::new(X11Connection { display }),
            }
        }
    }

    /// Create a new DeviceState. In case of failure, doesn't panic.
    pub fn checked_new() -> Option<DeviceState> {
        unsafe {
            let display = xlib::XOpenDisplay(ptr::null());
            if display.as_ref().is_none() {
                eprintln!("Could not connect to a X display");
                return None;
            }
            Some(DeviceState {
                xc: Rc::new(X11Connection { display }),
            })
        }
    }

    /// Query the `MouseState`.
    pub fn query_pointer(&self) -> MouseState {
        let root;
        let mut root_x = 0;
        let mut root_y = 0;
        let mut win_x = 0;
        let mut win_y = 0;
        let mut root_return = 0;
        let mut child_return = 0;
        let mut mask_return = 0;
        unsafe {
            root = xlib::XDefaultRootWindow(self.xc.display);
            xlib::XQueryPointer(
                self.xc.display,
                root,
                &mut root_return,
                &mut child_return,
                &mut root_x,
                &mut root_y,
                &mut win_x,
                &mut win_y,
                &mut mask_return,
            );
        }
        let button1pressed = mask_return & xlib::Button1Mask > 0;
        let button2pressed = mask_return & xlib::Button2Mask > 0;
        let button3pressed = mask_return & xlib::Button3Mask > 0;
        let button4pressed = mask_return & xlib::Button4Mask > 0;
        let button5pressed = mask_return & xlib::Button5Mask > 0;

        let button_pressed = [
            button1pressed,
            button2pressed,
            button3pressed,
            button4pressed,
            button5pressed,
        ];
        MouseState {
            coords: (win_x, win_y),
            button_pressed,
        }
    }

    /// Query the Keyboard state.
    pub fn query_keymap(&self) -> Vec<Code> {
        let mut keycodes = vec![];
        unsafe {
            let keymap: *mut c_char = [0; 32].as_mut_ptr();
            xlib::XQueryKeymap(self.xc.display, keymap);
            for (ix, byte) in slice::from_raw_parts(keymap, 32).iter().enumerate() {
                for bit in 0_u8..8_u8 {
                    let bitmask = 1 << bit;
                    if byte & bitmask != 0 {
                        //x11 keycode uses kernel keycode with an offset of 8.
                        let x11_key = ix as u8 * 8 + bit;
                        let kernel_key = x11_key - 8;
                        if let Some(k) = self.kernel_key_to_keycode(kernel_key) {
                            keycodes.push(k)
                        }
                    }
                }
            }
        }
        keycodes
    }

    fn kernel_key_to_keycode(&self, kernel_code: u8) -> Option<Code> {
        match kernel_code as u16 {
            kernel_key::KEY_0 => Some(Code::Digit0),
            kernel_key::KEY_1 => Some(Code::Digit1),
            kernel_key::KEY_2 => Some(Code::Digit2),
            kernel_key::KEY_3 => Some(Code::Digit3),
            kernel_key::KEY_4 => Some(Code::Digit4),
            kernel_key::KEY_5 => Some(Code::Digit5),
            kernel_key::KEY_6 => Some(Code::Digit6),
            kernel_key::KEY_7 => Some(Code::Digit7),
            kernel_key::KEY_8 => Some(Code::Digit8),
            kernel_key::KEY_9 => Some(Code::Digit9),
            kernel_key::KEY_A => Some(Code::KeyA),
            kernel_key::KEY_B => Some(Code::KeyB),
            kernel_key::KEY_C => Some(Code::KeyC),
            kernel_key::KEY_D => Some(Code::KeyD),
            kernel_key::KEY_E => Some(Code::KeyE),
            kernel_key::KEY_F => Some(Code::KeyF),
            kernel_key::KEY_G => Some(Code::KeyG),
            kernel_key::KEY_H => Some(Code::KeyH),
            kernel_key::KEY_I => Some(Code::KeyI),
            kernel_key::KEY_J => Some(Code::KeyJ),
            kernel_key::KEY_K => Some(Code::KeyK),
            kernel_key::KEY_L => Some(Code::KeyL),
            kernel_key::KEY_M => Some(Code::KeyM),
            kernel_key::KEY_N => Some(Code::KeyN),
            kernel_key::KEY_O => Some(Code::KeyO),
            kernel_key::KEY_P => Some(Code::KeyP),
            kernel_key::KEY_Q => Some(Code::KeyQ),
            kernel_key::KEY_R => Some(Code::KeyR),
            kernel_key::KEY_S => Some(Code::KeyS),
            kernel_key::KEY_T => Some(Code::KeyT),
            kernel_key::KEY_U => Some(Code::KeyU),
            kernel_key::KEY_V => Some(Code::KeyV),
            kernel_key::KEY_W => Some(Code::KeyW),
            kernel_key::KEY_X => Some(Code::KeyX),
            kernel_key::KEY_Y => Some(Code::KeyY),
            kernel_key::KEY_Z => Some(Code::KeyZ),
            kernel_key::KEY_F1 => Some(Code::F1),
            kernel_key::KEY_F2 => Some(Code::F2),
            kernel_key::KEY_F3 => Some(Code::F3),
            kernel_key::KEY_F4 => Some(Code::F4),
            kernel_key::KEY_F5 => Some(Code::F5),
            kernel_key::KEY_F6 => Some(Code::F6),
            kernel_key::KEY_F7 => Some(Code::F7),
            kernel_key::KEY_F8 => Some(Code::F8),
            kernel_key::KEY_F9 => Some(Code::F9),
            kernel_key::KEY_F10 => Some(Code::F10),
            kernel_key::KEY_F11 => Some(Code::F11),
            kernel_key::KEY_F12 => Some(Code::F12),
            kernel_key::KEY_F13 => Some(Code::F13),
            kernel_key::KEY_F14 => Some(Code::F14),
            kernel_key::KEY_F15 => Some(Code::F15),
            kernel_key::KEY_F16 => Some(Code::F16),
            kernel_key::KEY_F17 => Some(Code::F17),
            kernel_key::KEY_F18 => Some(Code::F18),
            kernel_key::KEY_F19 => Some(Code::F19),
            kernel_key::KEY_F20 => Some(Code::F20),
            kernel_key::KEY_KP0 => Some(Code::Numpad0),
            kernel_key::KEY_KP1 => Some(Code::Numpad1),
            kernel_key::KEY_KP2 => Some(Code::Numpad2),
            kernel_key::KEY_KP3 => Some(Code::Numpad3),
            kernel_key::KEY_KP4 => Some(Code::Numpad4),
            kernel_key::KEY_KP5 => Some(Code::Numpad5),
            kernel_key::KEY_KP6 => Some(Code::Numpad6),
            kernel_key::KEY_KP7 => Some(Code::Numpad7),
            kernel_key::KEY_KP8 => Some(Code::Numpad8),
            kernel_key::KEY_KP9 => Some(Code::Numpad9),
            kernel_key::KEY_KPENTER => Some(Code::NumpadEnter),
            kernel_key::KEY_KPMINUS => Some(Code::NumpadSubtract),
            kernel_key::KEY_KPPLUS => Some(Code::NumpadAdd),
            kernel_key::KEY_KPSLASH => Some(Code::NumpadDivide),
            kernel_key::KEY_KPASTERISK => Some(Code::NumpadMultiply),
            kernel_key::KEY_KPEQUAL => Some(Code::NumpadEquals),
            kernel_key::KEY_KPDOT => Some(Code::NumpadDecimal),
            kernel_key::KEY_ESC => Some(Code::Escape),
            kernel_key::KEY_SPACE => Some(Code::Space),
            kernel_key::KEY_LEFTCTRL => Some(Code::LControl),
            kernel_key::KEY_RIGHTCTRL => Some(Code::RControl),
            kernel_key::KEY_LEFTSHIFT => Some(Code::LShift),
            kernel_key::KEY_RIGHTSHIFT => Some(Code::ShiftRight),
            kernel_key::KEY_LEFTALT => Some(Code::AltLeft),
            kernel_key::KEY_RIGHTALT => Some(Code::AltRight),
            kernel_key::KEY_LEFTMETA => Some(Code::MetaLeft),
            kernel_key::KEY_RIGHTMETA => Some(Code::MetaRight),
            kernel_key::KEY_ENTER => Some(Code::Enter),
            kernel_key::KEY_UP => Some(Code::ArrowUp),
            kernel_key::KEY_DOWN => Some(Code::ArrowDown),
            kernel_key::KEY_LEFT => Some(Code::ArrowLeft),
            kernel_key::KEY_RIGHT => Some(Code::ArrowRight),
            kernel_key::KEY_BACKSPACE => Some(Code::Backspace),
            kernel_key::KEY_CAPSLOCK => Some(Code::CapsLock),
            kernel_key::KEY_TAB => Some(Code::Tab),
            kernel_key::KEY_HOME => Some(Code::Home),
            kernel_key::KEY_END => Some(Code::End),
            kernel_key::KEY_PAGEUP => Some(Code::PageUp),
            kernel_key::KEY_PAGEDOWN => Some(Code::PageDown),
            kernel_key::KEY_INSERT => Some(Code::Insert),
            kernel_key::KEY_DELETE => Some(Code::Delete),
            kernel_key::KEY_GRAVE => Some(Code::Backquote),
            kernel_key::KEY_MINUS => Some(Code::Minus),
            kernel_key::KEY_EQUAL => Some(Code::Equal),
            kernel_key::KEY_LEFTBRACE => Some(Code::BracketLeft),
            kernel_key::KEY_RIGHTBRACE => Some(Code::BracketRight),
            kernel_key::KEY_BACKSLASH => Some(Code::Backslash),
            kernel_key::KEY_SEMICOLON => Some(Code::Semicolon),
            kernel_key::KEY_APOSTROPHE => Some(Code::Quote),
            kernel_key::KEY_COMMA => Some(Code::Comma),
            kernel_key::KEY_DOT => Some(Code::Period),
            kernel_key::KEY_SLASH => Some(Code::Slash),
            _ => None,
        }
    }
}
