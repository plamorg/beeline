use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::utils::Duration;

use crate::camera::MainCamera;
use crate::player::Player;
use crate::ui::GameFont;
use crate::world::{spawn_world, GameWorld};
use crate::AppState;

pub struct StartDelayPlugin;

impl Plugin for StartDelayPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::StartDelay)
                .with_system(create_delay_timer)
                .with_system(spawn_world),
        )
        .add_system_set(SystemSet::on_update(AppState::StartDelay).with_system(update_delay_timer));
    }
}

const TIMER_SECS: f32 = 3.;

#[derive(Component)]
struct CameraSpeed {
    speed: Option<Vec2>,
}

fn create_delay_timer(mut commands: Commands, font: Res<GameFont>) {
    println!("enter create delay state");
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Percent(45.0),
                    left: Val::Percent(45.0),
                    ..Rect::default()
                },
                ..Style::default()
            },
            text: Text::with_section(
                "",
                TextStyle {
                    font: font.get_handle(),
                    font_size: 50.0,
                    ..TextStyle::default()
                },
                TextAlignment::default(),
            ),
            ..TextBundle::default()
        })
        .insert(Timer::new(Duration::from_secs(TIMER_SECS as u64), false))
        .insert(CameraSpeed { speed: None });
}

fn update_delay_timer(
    mut camera: Query<&mut Transform, With<MainCamera>>,
    player_transform: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut state: ResMut<State<AppState>>,
    world: Res<GameWorld>,
    time: Res<Time>,
    mut text: Query<(&mut Text, &mut Timer, &mut CameraSpeed)>,
) {
    let (mut text, mut timer, mut camera_speed) = text.single_mut();
    timer.tick(time.delta());
    if timer.finished() {
        println!("start delay finished");
        state.set(AppState::Game).unwrap();
        return;
    }
    let secs = TIMER_SECS - timer.elapsed_secs();
    text.sections[0].value = format!("{secs:.2}");

    const CAMERA_SECS: f32 = (TIMER_SECS) * (2./3.);

    // Lerp camera to player position
    let mut camera_transform = camera.single_mut();
    let player_transform = player_transform.single();
    if timer.elapsed_secs() < CAMERA_SECS {
        let cam_speed;
        if let Some(speed) = camera_speed.speed {
            cam_speed = speed;
        } else {
            cam_speed = (player_transform.translation.truncate() - camera_transform.translation.truncate())
                / CAMERA_SECS;
            camera_speed.speed = Some(cam_speed);
        }
        fn delta_distance(t: f32, dt: f32) -> f32 {
            // The velocity function we're using is
            // v(t) = -cos(2*pi*t) + 1
            // We need to evaluate integral of v(a) from a = t to a = t + dt,
            // which is -sin(2*pi*a)/(2*pi) + a evaluated from a = t to a = t + dt.
            fn indefinite_integral(t: f32) -> f32 {
                -(2. * PI * t).sin() / (2. * PI) + t
            }
            // In this way, the total distance traveled should be 1.
            indefinite_integral(t + dt) - indefinite_integral(t)
        }
        let ds = delta_distance(timer.elapsed_secs() / CAMERA_SECS, time.delta_seconds() / CAMERA_SECS);
        camera_transform.translation = (camera_transform.translation.truncate() + cam_speed * CAMERA_SECS * ds)
            .extend(camera_transform.translation.z);
    } else {
        camera_transform.translation = player_transform.translation;
    }
}
