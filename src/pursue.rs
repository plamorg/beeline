use crate::util::polar_to_cartesian;
use bevy::math::Vec2;

/// Returns the transform change that should be applied, *without* delta-time.
/// Pursue level 1, i.e. velocity to hit a stationary target.
pub fn pursue(pos: Vec2, target: Vec2, speed: f32) -> Vec2 {
    let relative_pos = target - pos;
    let velocity_angle = relative_pos.y.atan2(relative_pos.x);
    let velocity_scale = speed;
    polar_to_cartesian(velocity_angle, velocity_scale)
}
