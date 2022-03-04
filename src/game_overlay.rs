use bevy::utils::Instant;

use crate::{ui::GameFont, upgrades::create_upgrades_overlay, AppState};
use bevy::prelude::*;

pub struct GameOverlayPlugin;

impl Plugin for GameOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(create_game_overlay))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(update_game_timer));
    }
}

#[derive(Component)]
struct GameTimer(Instant);

impl Default for GameTimer {
    fn default() -> Self {
        Self(Instant::now())
    }
}

fn create_game_overlay(mut commands: Commands, font: Res<GameFont>) {
    commands.spawn_bundle(UiCameraBundle::default());
    create_upgrades_overlay(&mut commands, &font);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Percent(2.0),
                    left: Val::Percent(2.0),
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
        .insert(GameTimer::default());
}

fn update_game_timer(mut text: Query<(&mut Text, &GameTimer)>) {
    let (mut text, timer) = text.single_mut();
    let time = (Instant::now() - timer.0).as_secs_f32();
    text.sections[0].value = format!("{time:.2}");
}
