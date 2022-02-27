use crate::{player, AppState};
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

#[derive(Debug, Component, Clone)]
enum Projectile {
    Missile(f32),
    Laser(f32),
}

#[derive(Debug)]
enum Tile {
    Wall,
    Spawner(Projectile),
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
                    'L' => Some(Tile::Spawner(Projectile::Laser(
                        (&value[2..]).parse::<f32>().unwrap(),
                    ))),
                    'M' => Some(Tile::Spawner(Projectile::Missile(
                        (&value[2..]).parse::<f32>().unwrap(),
                    ))),
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
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_world));
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
                Some(Tile::Spawner(projectile)) => {
                    // TODO: Set sprite differently depending on projectile (rather than color)
                    let color = match projectile {
                        Projectile::Laser(_) => Color::GREEN,
                        Projectile::Missile(_) => Color::CYAN,
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
                        // Add projectile component so it can be queried later
                        .insert(projectile.clone());
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
