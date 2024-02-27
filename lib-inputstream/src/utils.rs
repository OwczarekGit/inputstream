#[inline]
pub fn map_value_to_new_range(
    value: f32,
    old_min: f32,
    old_max: f32,
    new_min: f32,
    new_max: f32,
) -> f32 {
    (value - old_min) * (new_max - new_min) / (old_max - old_min) + new_min
}

/// Dualsense uses numbers in range 0-255 for each axis
/// where 127 is neutral position.
#[inline]
pub fn map_to_dualsense_range(value: f32, min: f32, max: f32) -> f32 {
    (value - min) * 255.0 / (max - min)
}

#[cfg(feature = "sdl2")]
#[inline]
pub fn sdl2_to_dualsense_triggers(value: i16) -> u8 {
    (value as f32 * 255.0 / 32767.0).round() as u8
}

#[cfg(feature = "sdl2")]
#[inline]
pub fn sdl2_to_dualsense_sticks(value: f32) -> f32 {
    map_value_to_new_range(value, -32768.0, 32767.0, 0.0, 255.0)
}
