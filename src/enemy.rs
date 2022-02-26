use crate::{player::Player, pursue::pursue, AppState};
use benimator::{Play, SpriteSheetAnimation};
use bevy::prelude::*;
use std::{f32::consts::PI, time::Duration};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_enemy))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(move_enemy));
    }
}

#[derive(Component)]
pub struct Enemy;

impl Enemy {
    const SIZE: f32 = 24.0;
    const VELOCITY: f32 = 400.0;
}

fn spawn_enemy(
    mut commands: Commands,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
    mut textures: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    // Define enemy size
    let size = Vec2::splat(Enemy::SIZE);

    let animation_handle = animations.add(SpriteSheetAnimation::from_range(
        0..=7,
        Duration::from_millis(100),
    ));

    // Spawn enemy
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.add(TextureAtlas::from_grid(
                asset_server.load("rocket.png"),
                size,
                8,
                1,
            )),
            ..SpriteSheetBundle::default()
        })
        .insert(animation_handle)
        .insert(Play)
        .insert(Enemy);
}

fn move_enemy(
    time: Res<Time>,
    player_transform: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut transform: Query<&mut Transform, With<Enemy>>,
) {
    let player_transform = player_transform.single();
    let mut transform = transform.single_mut();
    let velocity = pursue(
        transform.translation.truncate(),
        player_transform.translation.truncate(),
        Enemy::VELOCITY,
    ) * time.delta_seconds();
    transform.translation.x += velocity.x;
    transform.translation.y += velocity.y;

    transform.rotation = Quat::from_rotation_z(-velocity.angle_between(Vec2::AXES[0]) - PI / 2.0);
}
