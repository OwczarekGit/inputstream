pub const DEFAULT_PORT: u16 = 2137;

pub const OSU_DEVICE_NAME: &str = "InputStream osu! input";
pub const MOUSE_DEVICE_NAME: &str = "InputStream mouse input";
pub const KEYBOARD_DEVICE_NAME: &str = "InputStream keyboard input";
pub const GAMEPAD_DEVICE_NAME: &str = "Xbox 360 Wireless Receiver";

/// Theese are here for compatibility.
/// Don't use them, they are inefficient.
///
/// Use `crate::event::Protocol::Osu` enum.
pub const OSU_PROTOCOL_NAME: &str = "OSU";
/// Use `crate::event::Protocol::Mouse` enum.
pub const MOUSE_PROTOCOL_NAME: &str = "MOUSE";
/// Use `crate::event::Protocol::Keyboard` enum.
pub const KEYBOARD_PROTOCOL_NAME: &str = "KEYBOARD";
/// Use `crate::event::Protocol::Gamepad` enum.
pub const GAMEPAD_PROTOCOL_NAME: &str = "GAMEPAD";
