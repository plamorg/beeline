use crate::{
    player::Player,
    upgrades::{Upgrade, UpgradeTracker},
    AppState,
};
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(follow_player));
    }
}

#[derive(Component)]
pub struct MainCamera;

impl MainCamera {
    const INTERPOLATION: f32 = 0.1;
}

pub fn spawn_camera(commands: &mut Commands, position: Vec2) {
    let mut orthographic_camera_bundle = OrthographicCameraBundle::new_2d();
    orthographic_camera_bundle.orthographic_projection.scale = 0.5;

    orthographic_camera_bundle.transform.translation.x = position.x;
    orthographic_camera_bundle.transform.translation.y = position.y;

    commands
        .spawn_bundle(orthographic_camera_bundle)
        .insert(MainCamera);
}

fn follow_player(
    mut camera_transform: Query<&mut Transform, With<MainCamera>>,
    player_transform: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    upgrades: Res<UpgradeTracker>,
) {
    let mut camera_transform = camera_transform.single_mut();
    let player_transform = player_transform.single();

    let interpolation = MainCamera::INTERPOLATION
        // Double interpolation if double speed is active
        * if upgrades.has_upgrade(Upgrade::DoubleSpeed) {
            2.0
        } else {
            1.0
        };

    camera_transform.translation = camera_transform
        .translation
        .truncate()
        .lerp(player_transform.translation.truncate(), interpolation)
        .extend(camera_transform.translation.z);
}
