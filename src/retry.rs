use crate::{
    ui::GameFont,
    world::{GameWorld, WorldType},
    AppState,
};
use bevy::prelude::*;

pub struct RetryPlugin;

impl Plugin for RetryPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Retry).with_system(create_retry_menu))
            .add_system_set(
                SystemSet::on_update(AppState::Retry).with_system(manage_retry_buttons),
            );
    }
}

#[derive(Component)]
enum ButtonType {
    Retry,
    Menu,
}

fn create_retry_menu(mut commands: Commands, font: Res<GameFont>) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..Style::default()
            },
            color: Color::NONE.into(),
            ..NodeBundle::default()
        })
        .with_children(|parent| {
            // Spawn upgrade select title
            parent.spawn_bundle(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Percent(5.0),
                        ..Rect::default()
                    },
                    ..Style::default()
                },
                text: Text::with_section(
                    "You died",
                    TextStyle {
                        font: font.get_handle(),
                        font_size: 90.0,
                        ..TextStyle::default()
                    },
                    TextAlignment::default(),
                ),
                ..TextBundle::default()
            });

            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: Rect {
                            top: Val::Percent(40.0),
                            ..Rect::default()
                        },
                        size: Size::new(Val::Px(300.0), Val::Px(65.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Style::default()
                    },
                    ..ButtonBundle::default()
                })
                .insert(ButtonType::Retry)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Retry",
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

            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: Rect {
                            top: Val::Percent(60.0),
                            ..Rect::default()
                        },
                        size: Size::new(Val::Px(300.0), Val::Px(65.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Style::default()
                    },
                    ..ButtonBundle::default()
                })
                .insert(ButtonType::Menu)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Main Menu",
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
        });
}

fn manage_retry_buttons(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    world: Res<GameWorld>,
    interaction: Query<(&Interaction, &ButtonType), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, button_type) in interaction.iter() {
        match (interaction, button_type) {
            (Interaction::Clicked, ButtonType::Retry) => {
                if let WorldType::Level { index, path } = &world.world_type {
                    commands.insert_resource(GameWorld::load_level(path, *index).unwrap());
                }

                state.set(AppState::Game).unwrap();
            }
            (Interaction::Clicked, ButtonType::Menu) => {
                state.set(AppState::Menu).unwrap();
            }
            _ => {}
        }
    }
}
