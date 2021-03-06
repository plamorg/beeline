use crate::{
    ui::{
        spawn_back_button, GameFont, ACTIVE_BUTTON_COLOR, INACTIVE_BUTTON_COLOR,
        NORMAL_BUTTON_COLOR,
    },
    upgrades::{create_upgrades_overlay, Upgrade, UpgradeTracker},
    AppState,
};
use bevy::prelude::*;
use strum::IntoEnumIterator;

pub struct UpgradeSelectPlugin;

impl Plugin for UpgradeSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::UpgradeSelect).with_system(create_upgrade_select),
        )
        .add_system_set(
            SystemSet::on_update(AppState::UpgradeSelect)
                .with_system(manage_upgrade_buttons)
                .with_system(update_upgrades_selected_indicator),
        );
    }
}

#[derive(Component)]
pub struct UpgradeButton(Upgrade);

// Component to display how many upgrades are currently selected
#[derive(Component)]
struct UpgradesSelectedIndicator;

fn create_upgrade_select(mut commands: Commands, font: Res<GameFont>) {
    commands.spawn_bundle(UiCameraBundle::default());

    spawn_back_button(&mut commands, font.get_handle());
    create_upgrades_overlay(&mut commands, &font);

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
                    position: Rect {
                        top: Val::Percent(2.0),
                        ..Rect::default()
                    },
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

            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: Rect {
                            bottom: Val::Percent(2.0),
                            right: Val::Percent(2.0),
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
                .insert(UpgradesSelectedIndicator);

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
                    let upgrades: Vec<Upgrade> = Upgrade::iter().collect();
                    for i in (0..upgrades.len()).step_by(3) {
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
                                    if i + j < upgrades.len() {
                                        let upgrade = upgrades[i + j];

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
                                                        upgrade.to_string(),
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
    mut upgrades: ResMut<UpgradeTracker>,
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

fn update_upgrades_selected_indicator(
    upgrades: Res<UpgradeTracker>,
    mut indicator: Query<&mut Text, With<UpgradesSelectedIndicator>>,
) {
    let number = match (upgrades.primary, upgrades.secondary) {
        (None, None) => "0",
        (Some(_), Some(_)) => "2",
        _ => "1",
    };

    let mut indicator = indicator.single_mut();
    indicator.sections[0].value = format!("{number}/2 upgrades selected");
}
