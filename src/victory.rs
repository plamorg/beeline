use crate::{ui::GameFont, AppState};
use bevy::prelude::*;

pub struct VictoryPlugin;

impl Plugin for VictoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Victory).with_system(create_victory_menu))
            .add_system_set(
                SystemSet::on_update(AppState::Victory).with_system(manage_menu_button),
            );
    }
}

#[derive(Component)]
struct MenuButton;

fn create_victory_menu(mut commands: Commands, font: Res<GameFont>) {
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
            parent.spawn_bundle(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Percent(20.0),
                        ..Rect::default()
                    },
                    ..Style::default()
                },
                text: Text::with_section(
                    "Victory!",
                    TextStyle {
                        font: font.get_handle(),
                        font_size: 120.0,
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
                .insert(MenuButton)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Menu",
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

fn manage_menu_button(
    mut state: ResMut<State<AppState>>,
    interaction: Query<&Interaction, (Changed<Interaction>, With<Button>, With<MenuButton>)>,
) {
    for interaction in interaction.iter() {
        if matches!(interaction, Interaction::Clicked) {
            state.set(AppState::Menu).unwrap();
        }
    }
}
