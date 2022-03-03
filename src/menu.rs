use crate::{
    player::Player,
    ui::GameFont,
    util::{AnimatedSprite, AnimatedSpriteData},
    AppState,
};
use benimator::SpriteSheetAnimation;
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(create_menu))
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(manage_menu_buttons));
    }
}

#[derive(Component)]
enum ButtonType {
    Play,
    Upgrades,
    Help,
}

fn create_menu(
    mut commands: Commands,
    font: Res<GameFont>,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
    mut textures: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Spawn player sprite
    commands.spawn_bundle(AnimatedSprite::new(
        &mut animations,
        &mut textures,
        &asset_server,
        AnimatedSpriteData {
            path: "bee.png".into(),
            frames: 6,
            size: Vec2::splat(Player::SIZE),
            transform: Transform::from_scale(Vec3::new(3.0, 3.0, 0.0)),
            ..AnimatedSpriteData::default()
        },
    ));

    commands.spawn_bundle(UiCameraBundle::default());
    commands.insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.4)));

    commands.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Percent(10.0),
                bottom: Val::Percent(60.0),
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
                    bottom: Val::Percent(45.0),
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
                    bottom: Val::Percent(30.0),
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

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Percent(10.0),
                    bottom: Val::Percent(15.0),
                    ..Rect::default()
                },
                size: Size::new(Val::Px(200.0), Val::Px(65.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Style::default()
            },
            ..ButtonBundle::default()
        })
        .insert(ButtonType::Help)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Help",
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

fn manage_menu_buttons(
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
            (Interaction::Clicked, ButtonType::Help) => {
                state.set(AppState::Help).unwrap();
            }
            _ => {}
        }
    }
}
