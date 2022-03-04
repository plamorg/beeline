use crate::{ui::GameFont, AppState};
use bevy::prelude::*;
use std::string::ToString;
use strum_macros::{Display, EnumIter};

const NO_UPGRADE_TEXT: &str = "EMPTY";

pub struct UpgradesPlugin;

impl Plugin for UpgradesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UpgradeTracker>()
            .add_system_set(
                SystemSet::on_update(AppState::UpgradeSelect).with_system(update_upgrades_overlay),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Game).with_system(update_upgrades_overlay),
            );
    }
}

#[derive(Component)]
struct UpgradeOverlay(UpgradeSlot);

pub fn create_upgrades_overlay(commands: &mut Commands, font: &Res<GameFont>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Percent(2.0),
                    bottom: Val::Percent(2.0),
                    ..Rect::default()
                },
                ..Style::default()
            },
            color: Color::NONE.into(),
            ..NodeBundle::default()
        })
        .with_children(|parent| {
            let text_bundle = TextBundle {
                text: Text::with_section(
                    NO_UPGRADE_TEXT,
                    TextStyle {
                        font: font.get_handle(),
                        font_size: 40.0,
                        ..TextStyle::default()
                    },
                    TextAlignment::default(),
                ),
                ..TextBundle::default()
            };

            parent
                .spawn_bundle(NodeBundle {
                    color: Color::GRAY.into(),
                    style: Style {
                        padding: Rect::all(Val::Px(10.0)),
                        ..Style::default()
                    },
                    ..NodeBundle::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(text_bundle.clone())
                        .insert(UpgradeOverlay(UpgradeSlot::Primary));
                });

            parent
                .spawn_bundle(NodeBundle {
                    color: Color::GRAY.into(),
                    style: Style {
                        padding: Rect::all(Val::Px(10.0)),
                        margin: Rect {
                            left: Val::Px(30.0),
                            ..Rect::default()
                        },
                        ..Style::default()
                    },
                    ..NodeBundle::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(text_bundle)
                        .insert(UpgradeOverlay(UpgradeSlot::Secondary));
                });
        });
}

fn update_upgrades_overlay(
    upgrades: Res<UpgradeTracker>,
    mut overlay_texts: Query<&mut Text, With<UpgradeOverlay>>,
) {
    let mut overlay_texts: Vec<Mut<'_, Text>> = overlay_texts.iter_mut().collect();
    assert_eq!(overlay_texts.len(), 2);

    overlay_texts[0].sections[0].value = if let Some(upgrade) = upgrades.primary {
        upgrade.to_string()
    } else {
        NO_UPGRADE_TEXT.into()
    };

    overlay_texts[1].sections[0].value = if let Some(upgrade) = upgrades.secondary {
        upgrade.to_string()
    } else {
        NO_UPGRADE_TEXT.into()
    };
}

#[derive(Debug, Display, EnumIter, PartialEq, Copy, Clone)]
pub enum Upgrade {
    DoubleSpeed,
    Shrink,
    Teleport,
    SlowEnemies,
}

pub enum UpgradeSlot {
    Primary,
    Secondary,
}

#[derive(Debug, Default)]
pub struct UpgradeTracker {
    pub primary: Option<Upgrade>,
    pub secondary: Option<Upgrade>,
}

impl UpgradeTracker {
    pub fn set_upgrade(&mut self, upgrade: Upgrade) {
        if self.primary == None {
            self.primary = Some(upgrade);
        } else if self.secondary == None {
            self.secondary = Some(upgrade);
        }
    }

    pub fn unset_upgrade(&mut self, upgrade: Upgrade) {
        if self.primary == Some(upgrade) {
            self.primary = None;
        } else if self.secondary == Some(upgrade) {
            self.secondary = None;
        }
    }

    pub fn was_upgrade_activated(
        &self,
        keyboard_input: Res<Input<KeyCode>>,
        button_input: Res<Input<MouseButton>>,
        upgrade: Upgrade,
    ) -> bool {
        (self.primary == Some(upgrade)
            && (button_input.just_pressed(MouseButton::Left)
                || keyboard_input.just_pressed(KeyCode::Q)))
            || (self.secondary == Some(upgrade)
                && (button_input.just_pressed(MouseButton::Right)
                    || keyboard_input.just_pressed(KeyCode::E)))
    }

    pub fn has_upgrade(&self, upgrade: Upgrade) -> bool {
        self.primary == Some(upgrade) || self.secondary == Some(upgrade)
    }
}
