use bevy::prelude::*;

use crate::{
    camera::MainCamera,
    util::{partial_min, polar_to_cartesian},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player).add_system(move_player);
    }
}

#[derive(Component)]
pub struct Player;

impl Player {
    const SIZE: f32 = 32.0;
    const VELOCITY: f32 = 500.0;
}

fn spawn_player(mut commands: Commands) {
    // Define player size
    let size = Vec2::splat(Player::SIZE);

    // Spawn player
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW,
                custom_size: Some(size),
                ..Sprite::default()
            },
            ..SpriteBundle::default()
        })
        .insert(Player);
}

fn move_player(
    windows: Res<Windows>,
    time: Res<Time>,
    camera: Query<&Camera, With<MainCamera>>,
    mut transform: Query<&mut Transform, With<Player>>,
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
        let magnitude_cap = partial_min(window.width(), window.height()) / 4.;
        // between 0 and 1
        let velocity_scale = partial_min(relative_pos.length(), magnitude_cap) / magnitude_cap;

        let velocity = polar_to_cartesian(velocity_angle, velocity_scale * Player::VELOCITY)
            * time.delta_seconds();
        let mut transform = transform.single_mut();
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}
