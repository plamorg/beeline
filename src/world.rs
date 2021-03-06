use crate::{
    camera,
    enemy::{Enemy, Projectile, Wall},
    player,
    upgrades::UpgradeTracker,
    util::{AnimatedSprite, AnimatedSpriteData},
    AppState,
};
use benimator::SpriteSheetAnimation;
use bevy::prelude::*;
use impacted::CollisionShape;
use std::f32::consts::PI;
use std::io;

pub enum WorldType {
    Level { index: usize },
    Endless,
}

#[derive(Component, Clone, Debug)]
pub struct Spawner {
    projectile: Projectile,
    timer: Timer,
}

impl Spawner {
    // Create spawner given a projectile enemy
    fn new(projectile: Projectile) -> Self {
        let cooldown = match projectile {
            Projectile::Missile => Projectile::MISSILE_COOLDOWN,
            Projectile::Laser { .. } => Projectile::LASER_COOLDOWN,
        };
        Self {
            projectile,
            timer: Timer::from_seconds(cooldown, true),
        }
    }
}

#[derive(Debug)]
pub enum Tile {
    Wall,
    Spawner(Spawner),
    Trap,
    Goal,
}

impl Tile {
    pub const SIZE: f32 = 24.0;
}

#[derive(Component)]
pub struct Goal;

pub struct GameWorld {
    pub world_type: WorldType,
    // Coordinates of the player's spawn location: (x, y)
    pub player_start_coordinates: (usize, usize),
    pub layout: Vec<Vec<Option<Tile>>>,
}

pub const LEVELS: [(&str, &str); 8] = [
    (
        "Closing Doors",
        include_str!("../assets/levels/Levels_-_Beeline_-_Closing_Doors.tsv"),
    ),
    ("Cornered", include_str!("../assets/levels/cornered.tsv")),
    (
        "Serpentine",
        include_str!("../assets/levels/Beeline_-_Serpentine.tsv"),
    ),
    (
        "Snakes on a Plane",
        include_str!("../assets/levels/snakes-on-a-plane.tsv"),
    ),
    ("Chicken", include_str!("../assets/levels/chicken.tsv")),
    ("Maze", include_str!("../assets/levels/maze.tsv")),
    (
        "Down The Road",
        include_str!("../assets/levels/down-the-road.tsv"),
    ),
    (
        "Drift",
        include_str!("../assets/levels/Levels_-_Beeline_-_Drift.tsv"),
    ),
];

impl GameWorld {
    pub fn load_level(level: usize) -> io::Result<Self> {
        // Open file and collect rows
        let lines = LEVELS[level].1.lines();

        let mut start = None;
        let mut layout = Vec::new();
        for (i, line) in lines.enumerate() {
            let mut row = Vec::new();
            for (j, value) in line.split('\t').enumerate() {
                let tile = value.chars().next().and_then(|char| match char {
                    '.' => None,
                    '#' => Some(Tile::Wall),
                    'L' => Some(Tile::Spawner(Spawner::new(Projectile::Laser {
                        angle: (&value[2..]).parse::<f32>().unwrap(),
                    }))),
                    'M' => Some(Tile::Spawner(Spawner::new(Projectile::Missile))),
                    'T' => Some(Tile::Trap),
                    'G' => Some(Tile::Goal),
                    '*' => {
                        // The * character indicates player's spawn location
                        start = Some((j, i));
                        None
                    }
                    _ => panic!("Invalid value: {value}"),
                });
                row.push(tile);
            }
            layout.push(row);
        }

        Ok(Self {
            world_type: WorldType::Level { index: level },
            player_start_coordinates: start.unwrap_or((0, 0)),
            layout,
        })
    }

    fn get_wall_neighbors(&self, x: usize, y: usize) -> [bool; 4] {
        let mut neighbors = [false; 4];
        let height = self.layout.len();
        let width = self.layout[y].len();

        let delta_y: [isize; 4] = [-1, 0, 1, 0];
        let delta_x: [isize; 4] = [0, 1, 0, -1];

        for i in 0..4 {
            let (nx, ny) = ((x as isize + delta_x[i]), (y as isize + delta_y[i]));

            if nx >= 0 && ny >= 0 && nx < width as isize && ny < height as isize {
                if let Some(Some(Tile::Wall)) = self.layout[ny as usize].get(nx as usize) {
                    neighbors[i] = true;
                    continue;
                }
            }
            neighbors[i] = false;
        }

        neighbors
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_world))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(spawn_projectiles));
    }
}

pub fn spawn_world(
    mut commands: Commands,
    world: Res<GameWorld>,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
    mut textures: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    upgrades: Res<UpgradeTracker>,
) {
    let tile_size = Vec2::splat(Tile::SIZE);
    let mut goal_position = None;

    // Iterate through the world layout and spawn tiles accordingly
    for (i, row) in world.layout.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            let transform =
                Transform::from_xyz(j as f32 * Tile::SIZE, -(i as f32 * Tile::SIZE), 0.0);
            match tile {
                Some(Tile::Wall) => {
                    let neighbors = world.get_wall_neighbors(j, i);
                    let name = match neighbors {
                        [true, false, true, false] => "wewe",
                        [false, false, true, false] => "eewe",
                        [true, false, true, true] => "weww",
                        [true, true, true, false] => "wwwe",
                        [false, true, false, true] => "ewew",
                        [false, false, false, true] => "eeew",
                        [false, true, false, false] => "ewee",
                        [true, true, false, true] => "wwew",
                        [true, true, true, true] => "wwww",
                        [true, false, false, false] => "weee",
                        [true, false, false, true] => "weew",
                        [true, true, false, false] => "wwee",
                        [false, true, true, true] => "ewww",
                        [false, true, true, false] => "ewwe",
                        [false, false, true, true] => "eeww",
                        [false, false, false, false] => "eeee",
                    };

                    let path = format!("walls/{name}.png");
                    commands
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(tile_size),
                                ..Sprite::default()
                            },
                            texture: asset_server.load(&path),
                            transform,
                            ..SpriteBundle::default()
                        })
                        .insert(CollisionShape::new_rectangle(tile_size.x, tile_size.y))
                        .insert(Wall)
                        .insert(Enemy);
                }
                Some(Tile::Spawner(spawner)) => match spawner.projectile {
                    Projectile::Missile => {
                        commands
                            .spawn_bundle(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(tile_size),
                                    ..Sprite::default()
                                },
                                texture: asset_server.load("missile-spawner.png"),
                                transform,
                                ..SpriteBundle::default()
                            })
                            .insert(spawner.clone());
                    }
                    Projectile::Laser { angle, .. } => {
                        commands
                            .spawn_bundle(AnimatedSprite::new(
                                &mut animations,
                                &mut textures,
                                &asset_server,
                                AnimatedSpriteData {
                                    path: "laser-spawner.png".into(),
                                    frames: 2,
                                    size: tile_size,
                                    transform: Transform {
                                        translation: transform.translation,
                                        rotation: Quat::from_rotation_z(angle - PI / 2.0),
                                        ..Transform::default()
                                    },
                                    ..AnimatedSpriteData::default()
                                },
                            ))
                            .insert(spawner.clone());
                    }
                },
                Some(Tile::Trap) => {
                    commands
                        .spawn_bundle(AnimatedSprite::new(
                            &mut animations,
                            &mut textures,
                            &asset_server,
                            AnimatedSpriteData {
                                path: "trap.png".into(),
                                frames: 6,
                                size: tile_size,
                                transform,
                                ..AnimatedSpriteData::default()
                            },
                        ))
                        .insert(CollisionShape::new_rectangle(tile_size.x, tile_size.y))
                        .insert(Enemy);
                }
                Some(Tile::Goal) => {
                    commands
                        .spawn_bundle(AnimatedSprite::new(
                            &mut animations,
                            &mut textures,
                            &asset_server,
                            AnimatedSpriteData {
                                path: "goal.png".into(),
                                frames: 6,
                                size: tile_size,
                                transform,
                                ..AnimatedSpriteData::default()
                            },
                        ))
                        .insert(CollisionShape::new_rectangle(tile_size.x, tile_size.y))
                        .insert(Goal);
                    goal_position = Some(transform.translation.truncate());
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
        &mut commands,
        animations,
        textures,
        &asset_server,
        upgrades,
        player_start_location,
    );

    camera::spawn_camera(&mut commands, goal_position.unwrap_or(Vec2::ZERO));
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

        if spawner.timer.tick(time.delta()).just_finished() {
            // Spawn projectile if timer has just finished
            spawner.projectile.spawn(
                &mut commands,
                &mut animations,
                &mut textures,
                &asset_server,
                spawn_position,
            );
        }
    }
}
