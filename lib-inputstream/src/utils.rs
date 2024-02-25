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
