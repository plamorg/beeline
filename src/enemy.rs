use crate::{
    player::Player,
    pursue::pursue,
    upgrades::{Upgrade, UpgradeTracker},
    util::polar_to_cartesian,
    util::{AnimatedSprite, AnimatedSpriteData},
    AppState,
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

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Clone, Debug)]
pub enum Projectile {
    Missile,
    Laser { angle: f32 },
}

impl Projectile {
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
            Projectile::Missile => {
                commands
                    .spawn_bundle(AnimatedSprite::new(
                        animations,
                        textures,
                        asset_server,
                        AnimatedSpriteData {
                            path: "rocket.png".into(),
                            frames: 8,
                            size: Self::MISSILE_SIZE.into(),
                            transform: Transform::from_translation(spawn_position),
                            ..AnimatedSpriteData::default()
                        },
                    ))
                    .insert(CollisionShape::new_rectangle(
                        Self::MISSILE_SIZE.0,
                        Self::MISSILE_SIZE.1,
                    ))
                    .insert(Pursuer::new(Self::MISSILE_VELOCITY))
                    .insert(self.clone())
                    .insert(Enemy);
            }
            Projectile::Laser { angle } => {
                commands
                    .spawn_bundle(AnimatedSprite::new(
                        animations,
                        textures,
                        asset_server,
                        AnimatedSpriteData {
                            path: "laser.png".into(),
                            frames: 4,
                            size: Self::LASER_SIZE.into(),
                            transform: Transform {
                                translation: spawn_position,
                                rotation: Quat::from_rotation_z(*angle - PI / 2.0),
                                // Spawn with zero scale
                                scale: Vec3::ZERO,
                            },
                            ..AnimatedSpriteData::default()
                        },
                    ))
                    .insert(CollisionShape::new_rectangle(
                        Self::LASER_SIZE.0,
                        Self::LASER_SIZE.1,
                    ))
                    .insert(Bullet::new(Self::LASER_VELOCITY, *angle))
                    .insert(self.clone())
                    .insert(Enemy);
            }
        }
    }
}

fn follow_player(
    time: Res<Time>,
    player_transform: Query<&Transform, (With<Player>, Without<Projectile>)>,
    mut enemies: Query<(&mut Transform, &Pursuer), With<Projectile>>,
    upgrades: Res<UpgradeTracker>,
) {
    for (mut transform, follow) in enemies.iter_mut() {
        let player_transform = player_transform.single();
        let velocity = pursue(
            transform.translation.truncate(),
            player_transform.translation.truncate(),
            follow.velocity,
        ) * time.delta_seconds()
            * if upgrades.has_upgrade(Upgrade::SlowEnemies) {
                0.5
            } else {
                1.0
            };
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;

        transform.rotation =
            Quat::from_rotation_z(-velocity.angle_between(Vec2::AXES[0]) - PI / 2.0);
    }
}

fn move_bullet_enemies(
    time: Res<Time>,
    mut enemies: Query<(&mut Transform, &Bullet), With<Projectile>>,
    upgrades: Res<UpgradeTracker>,
) {
    for (mut transform, bullet) in enemies.iter_mut() {
        transform.translation += (polar_to_cartesian(bullet.angle, 1.0)
            * bullet.velocity
            * time.delta_seconds()
            * if upgrades.has_upgrade(Upgrade::SlowEnemies) {
                0.5
            } else {
                1.0
            })
        .extend(0.0);
        transform.scale = transform.scale.lerp(Vec3::ONE, LASER_SCALE_INTERPOLATION);
    }
}
