#![allow(clippy::type_complexity)]

mod camera;
mod collision;
mod death;
mod enemy;
mod level_select;
mod menu;
mod player;
mod pursue;
mod upgrades;
mod util;
mod world;

use benimator::AnimationPlugin;
use bevy::prelude::*;

use camera::CameraPlugin;
use collision::CollisionPlugin;
use death::DeathPlugin;
use enemy::EnemyPlugin;
use level_select::LevelSelectPlugin;
use menu::MenuPlugin;
use player::PlayerPlugin;
use upgrades::UpgradesPlugin;
use world::WorldPlugin;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.65, 0.8, 0.44);
pub const ACTIVE_BUTTON_COLOR: Color = Color::rgb(0.98, 0.82, 0.48);

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum AppState {
    Menu,
    LevelSelect,
    Game,
    Death,
}

pub fn despawn_all(mut commands: Commands, entities: Query<Entity>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Menu)
        .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(despawn_all))
        .add_system_set(SystemSet::on_exit(AppState::LevelSelect).with_system(despawn_all))
        .add_system_set(SystemSet::on_exit(AppState::Death).with_system(despawn_all))
        .add_plugin(AnimationPlugin::default())
        .add_plugin(CameraPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(LevelSelectPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(UpgradesPlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(DeathPlugin)
        .run();
}
