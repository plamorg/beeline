use crate::{
    ui::{
        spawn_back_button, GameFont, ACTIVE_BUTTON_COLOR, INACTIVE_BUTTON_COLOR,
        NORMAL_BUTTON_COLOR,
    },
    upgrades::Upgrades,
    AppState,
};
use bevy::prelude::*;

pub struct UpgradeSelectPlugin;

impl Plugin for UpgradeSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::UpgradeSelect).with_system(create_upgrade_select),
        )
        .add_system_set(
            SystemSet::on_update(AppState::UpgradeSelect).with_system(manage_upgrade_buttons),
        );
    }
}

#[derive(Component)]
pub struct UpgradeButton(u64);

fn create_upgrade_select(mut commands: Commands, font: Res<GameFont>) {
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
            const UPGRADE_SELECT_HEIGHT: f32 = 50.0;
            // Spawn upgrade select title
            parent.spawn_bundle(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    ..Style::default()
                },
                text: Text::with_section(
                    "Upgrade Select",
                    TextStyle {
                        font: font.get_handle(),
                        font_size: 70.0,
                        ..TextStyle::default()
                    },
                    TextAlignment::default(),
                ),
                ..TextBundle::default()
            });

            // Spawn level selector
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(60.0), Val::Percent(UPGRADE_SELECT_HEIGHT)),
                        margin: Rect::all(Val::Auto),
                        flex_direction: FlexDirection::ColumnReverse,
                        ..Style::default()
                    },
                    color: Color::GRAY.into(),
                    ..NodeBundle::default()
                })
                .with_children(|parent| {
                    for i in (0..Upgrades::number_of_upgrades()).step_by(3) {
                        parent
                            .spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Auto, Val::Percent(30.0)),
                                    margin: Rect {
                                        top: Val::Px(5.0),
                                        bottom: Val::Px(5.0),
                                        ..Rect::default()
                                    },
                                    ..Style::default()
                                },
                                color: Color::GRAY.into(),
                                ..NodeBundle::default()
                            })
                            .with_children(|parent| {
                                for j in 0..3 {
                                    if i + j < Upgrades::number_of_upgrades() {
                                        let upgrade = 1 << (i + j);

                                        let upgrade_name = Upgrades::get_name(upgrade);

                                        parent
                                            .spawn_bundle(ButtonBundle {
                                                style: Style {
                                                    size: Size::new(
                                                        Val::Percent(30.0),
                                                        Val::Percent(100.0),
                                                    ),
                                                    margin: Rect::all(Val::Auto),
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::Center,
                                                    ..Style::default()
                                                },
                                                ..ButtonBundle::default()
                                            })
                                            .insert(UpgradeButton(upgrade))
                                            .with_children(|parent| {
                                                parent.spawn_bundle(TextBundle {
                                                    text: Text::with_section(
                                                        upgrade_name,
                                                        TextStyle {
                                                            font: font.get_handle(),
                                                            font_size: 30.0,
                                                            color: Color::BLACK,
                                                        },
                                                        TextAlignment::default(),
                                                    ),
                                                    ..TextBundle::default()
                                                });
                                            });
                                    } else {
                                        break;
                                    }
                                }
                            });
                    }
                });
        });
}

fn manage_upgrade_buttons(
    mut upgrades: ResMut<Upgrades>,
    mut interaction: Query<
        (&Interaction, &mut UiColor, &UpgradeButton),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, button) in interaction.iter_mut() {
        let has_upgrade = upgrades.has_upgrade(button.0);

        *color = if has_upgrade {
            INACTIVE_BUTTON_COLOR
        } else if matches!(interaction, Interaction::None) {
            NORMAL_BUTTON_COLOR
        } else {
            ACTIVE_BUTTON_COLOR
        }
        .into();

        if matches!(interaction, Interaction::Clicked) {
            if has_upgrade {
                upgrades.unset_upgrade(button.0);
            } else {
                upgrades.set_upgrade(button.0);
            }
        }
    }
}
