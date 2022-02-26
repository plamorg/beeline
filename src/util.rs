use bevy::math::Vec2;

/// Only use this when you know that the values
/// you're dealing with will produce a good result.
pub fn partial_min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

pub fn polar_to_cartesian(angle: f32, length: f32) -> Vec2 {
    Vec2::new(length * angle.cos(), length * angle.sin())
}
