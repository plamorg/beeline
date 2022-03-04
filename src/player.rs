use crate::{
    camera::MainCamera,
    enemy::Enemy,
    upgrades::{Upgrade, UpgradeTracker},
    util::{polar_to_cartesian, AnimatedSprite, AnimatedSpriteData},
    world::Goal,
    AppState,
};
use benimator::SpriteSheetAnimation;
use bevy::prelude::*;
use impacted::CollisionShape;
use std::f32::consts::PI;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct InvincibilityTimer(Timer);

impl Default for InvincibilityTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.2, false))
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Game).with_system(create_invincibility_timer),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(tick_invincibility_timer)
                .with_system(move_player)
                .with_system(detect_collision)
                .with_system(teleport),
        );
    }
}

fn create_invincibility_timer(mut commands: Commands) {
    commands.insert_resource(InvincibilityTimer::default());
}

fn tick_invincibility_timer(time: Res<Time>, mut timer: ResMut<InvincibilityTimer>) {
    timer.0.tick(time.delta());
}

#[derive(Component)]
pub struct Player;

impl Player {
    pub const SIZE: f32 = 24.0;
    const VELOCITY: f32 = 500.0;
}

// Spawn the player in the given start location
// This function should only be called by the world plugin
pub fn spawn_player(
    commands: &mut Commands,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
    mut textures: ResMut<Assets<TextureAtlas>>,
    asset_server: &Res<AssetServer>,
    upgrades: Res<UpgradeTracker>,
    start_location: Vec2,
) {
    // Define player size
    let size = Vec2::splat(Player::SIZE);

    let transform = Transform {
        translation: start_location.extend(1.0),
        scale: if upgrades.has_upgrade(Upgrade::Shrink) {
            // Half player scale if shrink upgrade is active
            Vec2::splat(0.5)
        } else {
            Vec2::ONE
        }
        .extend(1.0),
        ..Transform::default()
    };

    let collision_shape = if upgrades.has_upgrade(Upgrade::Shrink) {
        CollisionShape::new_rectangle(size.x / 2.0, size.y / 2.0)
    } else {
        CollisionShape::new_rectangle(size.x, size.y)
    };

    // Spawn player
    commands
        .spawn_bundle(AnimatedSprite::new(
            &mut animations,
            &mut textures,
            &asset_server,
            AnimatedSpriteData {
                path: "bee.png".into(),
                frames: 6,
                size,
                transform,
                ..AnimatedSpriteData::default()
            },
        ))
        .insert(collision_shape)
        .insert(Player);
}

fn move_player(
    windows: Res<Windows>,
    time: Res<Time>,
    upgrades: Res<UpgradeTracker>,
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
            * time.delta_seconds()
            * if upgrades.has_upgrade(Upgrade::DoubleSpeed) {
                // Double velocity if player has double speed upgrade
                2.0
            } else {
                1.0
            };

        let mut transform = transform.single_mut();
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;

        transform.rotation = Quat::from_rotation_z(velocity_angle - PI / 2.0);
    }
}

fn detect_collision(
    invincibility_timer: Res<InvincibilityTimer>,
    mut state: ResMut<State<AppState>>,
    enemies: Query<&CollisionShape, With<Enemy>>,
    goal: Query<&CollisionShape, With<Goal>>,
    player: Query<&CollisionShape, With<Player>>,
) {
    if invincibility_timer.0.finished() {
        if let Ok(player) = player.get_single() {
            for enemy in enemies.iter() {
                if player.is_collided_with(enemy) {
                    state.set(AppState::Death).unwrap();
                    return;
                }
            }
            if let Ok(goal) = goal.get_single() {
                if player.is_collided_with(goal) {
                    state.set(AppState::Victory).unwrap();
                }
            }
        }
    }
}

fn teleport(
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    button_input: Res<Input<MouseButton>>,
    upgrades: Res<UpgradeTracker>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    if upgrades.was_upgrade_activated(button_input, Upgrade::Teleport) {
        let (camera, camera_transform) = camera.single();
        let window = windows.get(camera.window).unwrap();

        if let Some(cursor_pos) = window.cursor_position() {
            // Calculate the cursor's world position
            let window_size = Vec2::new(window.width() as f32, window.height() as f32);
            let ndc = (cursor_pos / window_size) * 2.0 - Vec2::ONE;
            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix.inverse();
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0)).truncate();

            // Set player translation to the cursor's world position
            let mut player_transform = player.single_mut();
            player_transform.translation = world_pos.extend(player_transform.translation.z);
        }
    }
}
