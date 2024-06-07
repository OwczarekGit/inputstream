pub use crate::client::TcpClient;
pub use crate::event::difference::Difference;
pub use crate::event::gamepad::{
    absolute_axis::AbsoluteAxis, gamepad_button::GamepadButton, gamepad_state::GamepadState,
};
pub use crate::event::keyboard::named::Key;
pub use crate::event::keyboard::{KeyboardEventGroup, KeyboardState};
pub use crate::event::mouse::{MouseButton, MouseState};
pub use crate::event::osu::{OsuKey, OsuState};
pub use crate::event::protocol::Protocol;
pub use crate::event::EventType;
