use crate::dispatcher::message::MessageKind;

pub mod difference;
pub mod gamepad;
pub mod keyboard;
pub mod motion;
pub mod mouse;

pub const MOUSE_MESSAGE_KIND: MessageKind = MessageKind::MAX;
pub const KEYBOARD_MESSAGE_KIND: MessageKind = MessageKind::MAX - 1;
pub const GAMEPAD_MESSAGE_KIND: MessageKind = MessageKind::MAX - 2;
pub const MOTION_MESSAGE_KIND: MessageKind = MessageKind::MAX - 3;
