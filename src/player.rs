use crate::{
    camera::MainCamera,
    util::{polar_to_cartesian, AnimatedSprite},
    AppState,
};
use benimator::SpriteSheetAnimation;
use bevy::prelude::*;
use std::f32::consts::PI;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(move_player));
    }
}

#[derive(Component)]
pub struct Player;

impl Player {
    const SIZE: f32 = 24.0;
    const VELOCITY: f32 = 500.0;
}

// Spawn the player in the given start location
// This function should only be called by the world plugin
pub fn spawn_player(
    mut commands: Commands,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
    mut textures: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    start_location: Vec2,
) {
    // Define player size
    let size = Vec2::splat(Player::SIZE);

    // Spawn player
    commands
        .spawn_bundle(AnimatedSprite::new(
            &mut animations,
            &mut textures,
            &asset_server,
            "bee.png",
            6,
            size,
            start_location,
        ))
        .insert(Player);
}

fn move_player(
    windows: Res<Windows>,
    time: Res<Time>,
    camera: Query<&Camera, With<MainCamera>>,
    mut transform: Query<&mut Transform, (With<Player>, Without<MainCamera>)>,
) {
    let camera = camera.single();
    let window = windows.get(camera.window).unwrap();
    // Some(_) if the cursor is in the window
    if let Some(cursor_pos) = window.cursor_position() {
        let relative_pos = Vec2::new(
            cursor_pos.x - window.width() / 2.,
            cursor_pos.y - window.height() / 2.,
        );
        let velocity_angle = relative_pos.y.atan2(relative_pos.x);
        let magnitude_cap = window.width().min(window.height()) / 4.;
        // between 0 and 1
        let velocity_scale = relative_pos.length().min(magnitude_cap) / magnitude_cap;

        let velocity = polar_to_cartesian(velocity_angle, velocity_scale * Player::VELOCITY)
            * time.delta_seconds();
        let mut transform = transform.single_mut();
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;

        transform.rotation = Quat::from_rotation_z(velocity_angle - PI / 2.0);
    }
}
