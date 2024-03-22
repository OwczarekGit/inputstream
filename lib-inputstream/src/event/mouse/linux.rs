use super::MouseButton;

#[cfg(target_os = "linux")]
impl From<MouseButton> for evdev::Key {
    fn from(value: MouseButton) -> Self {
        use evdev::Key;
        match value {
            MouseButton::Left => Key::BTN_LEFT,
            MouseButton::Right => Key::BTN_RIGHT,
            MouseButton::Middle => Key::BTN_MIDDLE,
        }
    }
}
