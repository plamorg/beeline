use crate::{player::Player, AppState};
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_camera))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(follow_player));
    }
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    let mut orthographic_camera_bundle = OrthographicCameraBundle::new_2d();
    orthographic_camera_bundle.orthographic_projection.scale = 0.5;

    commands
        .spawn_bundle(orthographic_camera_bundle)
        .insert(MainCamera);
}

fn follow_player(
    mut camera_transform: Query<&mut Transform, With<MainCamera>>,
    player_transform: Query<&Transform, (With<Player>, Without<MainCamera>)>,
) {
    let mut camera_transform = camera_transform.single_mut();
    let player_transform = player_transform.single();

    camera_transform.translation = camera_transform
        .translation
        .truncate()
        .lerp(player_transform.translation.truncate(), 0.1)
        .extend(camera_transform.translation.z);
}
