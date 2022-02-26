use bevy::math::Vec2;

pub fn polar_to_cartesian(angle: f32, length: f32) -> Vec2 {
    Vec2::new(length * angle.cos(), length * angle.sin())
}
