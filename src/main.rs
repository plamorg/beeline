mod camera;
mod player;
mod util;

use benimator::AnimationPlugin;
use bevy::prelude::*;

use camera::CameraPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AnimationPlugin::default())
        .add_plugin(CameraPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
