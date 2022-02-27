use crate::{
    player::Player, pursue::pursue, util::polar_to_cartesian, util::AnimatedSprite, AppState,
};
use benimator::SpriteSheetAnimation;
use bevy::prelude::*;
use std::f32::consts::PI;

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
    Laser { velocity: f32, angle: f32 },
}

impl Enemy {
    pub fn new_laser(angle: f32) -> Self {
        Self::Laser {
            velocity: 800.0,
            angle,
        }
    }

    pub fn spawn(
        &self,
        commands: &mut Commands,
        animations: &mut ResMut<Assets<SpriteSheetAnimation>>,
        textures: &mut ResMut<Assets<TextureAtlas>>,
        asset_server: &Res<AssetServer>,
        spawn_position: Vec2,
    ) {
        match self {
            Enemy::Missile => {
                commands
                    .spawn_bundle(AnimatedSprite::new(
                        animations,
                        textures,
                        asset_server,
                        "rocket.png",
                        8,
                        Vec2::splat(24.0),
                        spawn_position,
                    ))
                    .insert(Pursuer::new(400.0))
                    .insert(self.clone());
            }
            Enemy::Laser { velocity, angle } => {
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::RED,
                            custom_size: Some(Vec2::splat(24.0)),
                            ..Sprite::default()
                        },
                        transform: Transform {
                            translation: spawn_position.extend(0.0),
                            rotation: Quat::from_rotation_z(*angle),
                            ..Transform::default()
                        },
                        ..SpriteBundle::default()
                    })
                    .insert(Bullet::new(*velocity, *angle))
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
    }
}
