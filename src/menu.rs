use crate::{ui::GameFont, AppState};
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(create_menu))
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(manage_menu_button));
    }
}

#[derive(Component)]
enum ButtonType {
    Play,
    Upgrades,
}

fn create_menu(mut commands: Commands, font: Res<GameFont>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands.insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.4)));

    commands.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Percent(10.0),
                bottom: Val::Percent(50.0),
                ..Rect::default()
            },
            ..Style::default()
        },
        text: Text::with_section(
            "Beeline",
            TextStyle {
                font: font.get_handle(),
                font_size: 130.0,
                ..TextStyle::default()
            },
            TextAlignment::default(),
        ),
        ..TextBundle::default()
    });

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Percent(10.0),
                    bottom: Val::Percent(35.0),
                    ..Rect::default()
                },
                size: Size::new(Val::Px(200.0), Val::Px(65.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Style::default()
            },
            ..ButtonBundle::default()
        })
        .insert(ButtonType::Play)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Play",
                    TextStyle {
                        font: font.get_handle(),
                        font_size: 60.0,
                        color: Color::BLACK,
                    },
                    TextAlignment::default(),
                ),
                ..TextBundle::default()
            });
        });

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Percent(10.0),
                    bottom: Val::Percent(20.0),
                    ..Rect::default()
                },
                size: Size::new(Val::Px(200.0), Val::Px(65.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Style::default()
            },
            ..ButtonBundle::default()
        })
        .insert(ButtonType::Upgrades)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Upgrades",
                    TextStyle {
                        font: font.get_handle(),
                        font_size: 60.0,
                        color: Color::BLACK,
                    },
                    TextAlignment::default(),
                ),
                ..TextBundle::default()
            });
        });
}

fn manage_menu_button(
    mut state: ResMut<State<AppState>>,
    interaction: Query<(&Interaction, &ButtonType), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, button_type) in interaction.iter() {
        match (interaction, button_type) {
            (Interaction::Clicked, ButtonType::Play) => {
                state.set(AppState::LevelSelect).unwrap();
            }
            (Interaction::Clicked, ButtonType::Upgrades) => {
                state.set(AppState::UpgradeSelect).unwrap();
            }
            _ => {}
        }
    }
}
