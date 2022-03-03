#![allow(clippy::type_complexity)]

mod camera;
mod collision;
mod death;
mod enemy;
mod game_overlay;
mod help;
mod level_select;
mod menu;
mod player;
mod pursue;
mod retry;
mod ui;
mod upgrade_select;
mod upgrades;
mod util;
mod world;

use benimator::AnimationPlugin;
use bevy::prelude::*;

use camera::CameraPlugin;
use collision::CollisionPlugin;
use death::DeathPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use ui::UiPlugins;
use upgrades::UpgradesPlugin;
use world::WorldPlugin;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum AppState {
    Menu,
    UpgradeSelect,
    LevelSelect,
    Help,
    Game,
    Death,
    Retry,
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
        .add_system_set(SystemSet::on_exit(AppState::UpgradeSelect).with_system(despawn_all))
        .add_system_set(SystemSet::on_exit(AppState::LevelSelect).with_system(despawn_all))
        .add_system_set(SystemSet::on_exit(AppState::Help).with_system(despawn_all))
        .add_system_set(SystemSet::on_exit(AppState::Retry).with_system(despawn_all))
        .add_plugin(AnimationPlugin::default())
        .add_plugin(CameraPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugins(UiPlugins)
        .add_plugin(UpgradesPlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(DeathPlugin)
        .run();
}
