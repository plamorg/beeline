use crate::{
    player::Player,
    util::{AnimatedSprite, AnimatedSpriteData},
    AppState,
};
use benimator::{AnimationMode, Play, SpriteSheetAnimation};
use bevy::prelude::*;
use rand::random;
use std::{f32::consts::PI, time::Duration};

pub struct DeathPlugin;

impl Plugin for DeathPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Death).with_system(spawn_death_anim))
            .add_system_set(
                SystemSet::on_update(AppState::Death)
                    .with_system(end_death_anim)
                    .with_system(update_flakes)
                    .with_system(update_death_timer),
            );
    }
}

#[derive(Component)]
struct DeathShard {
    sin_angle: f32,
    cos_angle: f32,
}

impl DeathShard {
    const SIZE: f32 = 24.0;
}

#[derive(Component)]
struct DeathTimer;

fn spawn_death_anim(
    mut commands: Commands,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
    player_info: Query<(Entity, &Transform), With<Player>>,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle((Timer::from_seconds(3.5, false), DeathTimer));

    let (player_entity, player_transform) = player_info.single();
    let player_transform = *player_transform;

    commands.entity(player_entity).despawn_recursive();

    commands
        .spawn_bundle(AnimatedSprite::new(
            &mut animations,
            &mut textures,
            &asset_server,
            AnimatedSpriteData {
                path: "bee-dead.png".into(),
                frames: 73,
                size: Vec2::splat(Player::SIZE),
                transform: player_transform,
                delay: Duration::from_millis(35),
                mode: AnimationMode::Once,
            },
        ))
        .insert(Player)
        .insert(Play);
}

const SHARD_SPEED: f32 = 7.;

fn end_death_anim(
    mut commands: Commands,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
    mut textures: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &Transform, Option<&Play>), With<Player>>,
) {
    if let Ok((player_entity, player_transform, playing)) = query.get_single() {
        let player_transform = *player_transform;
        if playing.is_none() {
            // the animation is no longer playing (AnimationMode::Once)
            // destroy the player death entity
            commands.entity(player_entity).despawn_recursive();
            // create death flakes
            const DEATH_SHARDS: usize = 6;
            for i in 0..DEATH_SHARDS {
                // [0, 2pi)
                let angle = random::<f32>() * 2. * PI;

                let sprite_path = if i % 2 == 0 {
                    "bee-shard-yellow.png"
                } else {
                    "bee-shard-brown.png"
                }
                .into();
                commands
                    .spawn_bundle(AnimatedSprite::new(
                        &mut animations,
                        &mut textures,
                        &asset_server,
                        AnimatedSpriteData {
                            path: sprite_path,
                            frames: 11,
                            size: Vec2::splat(DeathShard::SIZE),
                            transform: Transform::from_translation(player_transform.translation),
                            delay: Duration::from_millis(50),
                            ..AnimatedSpriteData::default()
                        },
                    ))
                    .insert(DeathShard {
                        sin_angle: angle.sin(),
                        cos_angle: angle.cos(),
                    });
            }
        }
    }
}

fn update_flakes(mut transform: Query<(&DeathShard, &mut Transform)>) {
    for (shard, mut transform) in transform.iter_mut() {
        let translation = &mut transform.translation;
        translation.y += shard.sin_angle * SHARD_SPEED;
        translation.x += shard.cos_angle * SHARD_SPEED;
    }
}

fn update_death_timer(
    mut state: ResMut<State<AppState>>,
    time: Res<Time>,
    mut timer: Query<&mut Timer, With<DeathTimer>>,
) {
    let mut timer = timer.single_mut();
    if timer.tick(time.delta()).just_finished() {
        state.set(AppState::Retry).unwrap();
    }
}
