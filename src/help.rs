use crate::{
    ui::{spawn_back_button, GameFont},
    AppState,
};
use bevy::prelude::*;

pub struct HelpPlugin;

impl Plugin for HelpPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Help).with_system(create_help));
    }
}

fn create_help(mut commands: Commands, font: Res<GameFont>) {
    commands.spawn_bundle(UiCameraBundle::default());

    spawn_back_button(&mut commands, font.get_handle());

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
                        top: Val::Percent(2.0),
                        ..Rect::default()
                    },
                    ..Style::default()
                },
                text: Text::with_section(
                    "Help",
                    TextStyle {
                        font: font.get_handle(),
                        font_size: 70.0,
                        ..TextStyle::default()
                    },
                    TextAlignment::default(),
                ),
                ..TextBundle::default()
            });

            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(70.0), Val::Percent(50.0)),
                        margin: Rect::all(Val::Auto),
                        align_items: AlignItems::Center,
                        ..Style::default()
                    },
                    color: Color::NONE.into(),
                    ..NodeBundle::default()
                })
                .with_children(|parent| {
                    let text = vec![
                        "Welcome to Beeline.\n",
                        "\n",
                        "Head over to the upgrades menu to gain an \"unfair\" advantage!\n",
                        "\n",
                        "Some upgrades can be activated using mouse buttons:\n",
                        "Left Click or Q - Use primary upgrade\n",
                        "Right Click or E - Use secondary upgrade",
                    ];

                    parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: text
                                .iter()
                                .map(|value| TextSection {
                                    value: value.to_string(),
                                    style: TextStyle {
                                        font: font.get_handle(),
                                        font_size: 40.0,
                                        ..TextStyle::default()
                                    },
                                })
                                .collect(),
                            ..Text::default()
                        },
                        ..TextBundle::default()
                    });
                });
        });
}
