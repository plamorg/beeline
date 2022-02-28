use crate::{
    player::Player, pursue::pursue, util::polar_to_cartesian, util::AnimatedSprite, AppState,
};
use benimator::SpriteSheetAnimation;
use bevy::prelude::*;
use impacted::CollisionShape;
use std::f32::consts::PI;

const LASER_SCALE_INTERPOLATION: f32 = 0.08;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(follow_player)
                .with_system(move_bullet_enemies),
        );
    }
}

#[derive(Component)]
struct Pursuer {
    velocity: f32,
}

impl Pursuer {
    fn new(velocity: f32) -> Self {
        Self { velocity }
    }
}

// Simple moving enemy, only travels in the given angle
#[derive(Component)]
struct Bullet {
    velocity: f32,
    angle: f32,
}

impl Bullet {
    fn new(velocity: f32, angle: f32) -> Self {
        Self { velocity, angle }
    }
}

#[derive(Component, Clone, Debug)]
pub enum Enemy {
    Missile,
    Laser { angle: f32 },
}

impl Enemy {
    const MISSILE_SIZE: (f32, f32) = (24.0, 24.0);
    const MISSILE_VELOCITY: f32 = 400.0;
    pub const MISSILE_COOLDOWN: f32 = 1.0;

    const LASER_SIZE: (f32, f32) = (12.0, 24.0);
    const LASER_VELOCITY: f32 = 300.0;
    pub const LASER_COOLDOWN: f32 = 0.1;

    pub fn spawn(
        &self,
        commands: &mut Commands,
        animations: &mut ResMut<Assets<SpriteSheetAnimation>>,
        textures: &mut ResMut<Assets<TextureAtlas>>,
        asset_server: &Res<AssetServer>,
        spawn_position: Vec2,
    ) {
        // Set z-ordering to 2.0 to ensure that enemies are spawned above the player and spawners
        let spawn_position = spawn_position.extend(2.0);
        match self {
            Enemy::Missile => {
                commands
                    .spawn_bundle(AnimatedSprite::new(
                        animations,
                        textures,
                        asset_server,
                        "rocket.png",
                        8,
                        Self::MISSILE_SIZE.into(),
                        Transform::from_translation(spawn_position),
                    ))
                    .insert(CollisionShape::new_rectangle(
                        Self::MISSILE_SIZE.0,
                        Self::MISSILE_SIZE.1,
                    ))
                    .insert(Pursuer::new(Self::MISSILE_VELOCITY))
                    .insert(self.clone());
            }
            Enemy::Laser { angle } => {
                commands
                    .spawn_bundle(AnimatedSprite::new(
                        animations,
                        textures,
                        asset_server,
                        "laser.png",
                        4,
                        Self::LASER_SIZE.into(),
                        Transform {
                            translation: spawn_position,
                            rotation: Quat::from_rotation_z(*angle - PI / 2.0),
                            scale: Vec3::ZERO,
                        },
                    ))
                    .insert(CollisionShape::new_rectangle(
                        Self::LASER_SIZE.0,
                        Self::LASER_SIZE.1,
                    ))
                    .insert(Bullet::new(Self::LASER_VELOCITY, *angle))
                    .insert(self.clone());
            }
        }
    }
}

fn follow_player(
    time: Res<Time>,
    player_transform: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemies: Query<(&mut Transform, &Pursuer), With<Enemy>>,
) {
    for (mut transform, follow) in enemies.iter_mut() {
        let player_transform = player_transform.single();
        let velocity = pursue(
            transform.translation.truncate(),
            player_transform.translation.truncate(),
            follow.velocity,
        ) * time.delta_seconds();
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;

        transform.rotation =
            Quat::from_rotation_z(-velocity.angle_between(Vec2::AXES[0]) - PI / 2.0);
    }
}

fn move_bullet_enemies(
    time: Res<Time>,
    mut enemies: Query<(&mut Transform, &Bullet), With<Enemy>>,
) {
    for (mut transform, bullet) in enemies.iter_mut() {
        transform.translation +=
            (polar_to_cartesian(bullet.angle, 1.0) * bullet.velocity * time.delta_seconds())
                .extend(0.0);
        transform.scale = transform.scale.lerp(Vec3::ONE, LASER_SCALE_INTERPOLATION);
    }
}
