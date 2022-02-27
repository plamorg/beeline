use crate::{enemy::Enemy, player, AppState};
use benimator::SpriteSheetAnimation;
use bevy::prelude::*;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

enum WorldType {
    Level(usize),
    Endless,
}

#[derive(Component, Clone, Debug)]
struct Spawner {
    enemy: Enemy,
    timer: Timer,
}

impl Spawner {
    // Create spawner given an enemy and cooldown (in seconds)
    fn new(enemy: Enemy, cooldown: f32) -> Self {
        Self {
            enemy,
            timer: Timer::from_seconds(cooldown, true),
        }
    }
}

#[derive(Debug)]
enum Tile {
    Wall,
    Spawner(Spawner),
}

impl Tile {
    const SIZE: f32 = 24.0;
}

pub struct World {
    world_type: WorldType,
    // Coordinates of the player's spawn location: (x, y)
    player_start_coordinates: (usize, usize),
    layout: Vec<Vec<Option<Tile>>>,
}

impl World {
    pub fn load_level<P: AsRef<Path>>(path: P, level: usize) -> io::Result<Self> {
        // Open file and collect rows
        let file = File::open(path)?;
        let lines: Vec<io::Result<String>> = BufReader::new(file).lines().collect();

        let mut start = None;
        let mut layout = Vec::new();
        for (i, line) in lines.iter().flatten().enumerate() {
            let mut row = Vec::new();
            for (j, value) in line.split('\t').enumerate() {
                let tile = match value.chars().next().unwrap() {
                    '.' => None,
                    '#' => Some(Tile::Wall),
                    'L' => Some(Tile::Spawner(Spawner::new(
                        Enemy::new_laser((&value[2..]).parse::<f32>().unwrap()),
                        0.1,
                    ))),
                    'M' => Some(Tile::Spawner(Spawner::new(Enemy::Missile, 1.0))),
                    '*' => {
                        // The * character indicates player's spawn location
                        start = Some((j, i));
                        None
                    }
                    _ => panic!("Invalid value: {value}"),
                };
                row.push(tile);
            }
            layout.push(row);
        }

        Ok(Self {
            world_type: WorldType::Level(level),
            player_start_coordinates: start.unwrap_or((0, 0)),
            layout,
        })
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_world))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(spawn_projectiles));
    }
}

fn spawn_world(
    mut commands: Commands,
    world: Res<World>,
    animations: ResMut<Assets<SpriteSheetAnimation>>,
    textures: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let tile_size = Vec2::splat(Tile::SIZE);

    // Iterate through the world layout and spawn tiles accordingly
    for (i, row) in world.layout.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            let transform =
                Transform::from_xyz(j as f32 * Tile::SIZE, -(i as f32 * Tile::SIZE), 0.0);
            match tile {
                Some(Tile::Wall) => {
                    commands.spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::RED,
                            custom_size: Some(tile_size),
                            ..Sprite::default()
                        },
                        transform,
                        ..SpriteBundle::default()
                    });
                }
                Some(Tile::Spawner(spawner)) => {
                    // TODO: Set sprite differently depending on projectile (rather than color)
                    let color = match spawner.enemy {
                        Enemy::Missile => Color::CYAN,
                        Enemy::Laser { .. } => Color::GREEN,
                    };
                    commands
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                color,
                                custom_size: Some(tile_size),
                                ..Sprite::default()
                            },
                            transform,
                            ..SpriteBundle::default()
                        })
                        .insert(spawner.clone());
                }
                None => {}
            }
        }
    }

    // Convert player start coordinates into world position
    let player_start_location = Vec2::new(
        world.player_start_coordinates.0 as f32,
        -(world.player_start_coordinates.1 as f32),
    ) * Tile::SIZE;

    // Spawn the player
    player::spawn_player(
        commands,
        animations,
        textures,
        asset_server,
        player_start_location,
    );
}

fn spawn_projectiles(
    mut commands: Commands,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
    mut textures: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut spawners: Query<(&Transform, &mut Spawner)>,
) {
    for (spawner_transform, mut spawner) in spawners.iter_mut() {
        let spawn_position = spawner_transform.translation.truncate();

        match spawner.enemy {
            Enemy::Missile => {
                if spawner.timer.tick(time.delta()).just_finished() {
                    Enemy::Missile.spawn(
                        &mut commands,
                        &mut animations,
                        &mut textures,
                        &asset_server,
                        spawn_position,
                    );
                }
            }
            Enemy::Laser { velocity, angle } => {
                if spawner.timer.tick(time.delta()).just_finished() {
                    Enemy::Laser { velocity, angle }.spawn(
                        &mut commands,
                        &mut animations,
                        &mut textures,
                        &asset_server,
                        spawn_position,
                    );
                }
            }
        }
    }
}
