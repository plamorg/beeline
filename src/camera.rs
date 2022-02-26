use crate::player::Player;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_system(follow_player);
    }
}

#[derive(Component)]
struct MainCamera;

impl MainCamera {
    // Interpolation value for following the player
    const INTERPOLATION: f32 = 0.1;
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}

fn follow_player(
    mut camera_transform: Query<&mut Transform, With<MainCamera>>,
    player_transform: Query<&Transform, (With<Player>, Without<MainCamera>)>,
) {
    let mut camera_transform = camera_transform.single_mut();
    let player_transform = player_transform.single();

    // Set camera translation without overwriting z-ordering
    camera_transform.translation = camera_transform
        .translation
        .truncate()
        .lerp(
            player_transform.translation.truncate(),
            MainCamera::INTERPOLATION,
        )
        .extend(0.0);
}
