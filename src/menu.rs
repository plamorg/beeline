use crate::{AppState, ACTIVE_BUTTON_COLOR, NORMAL_BUTTON_COLOR};
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(create_menu))
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(manage_menu_button));
    }
}

fn create_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands.insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.4)));

    let font = asset_server.load("FrancoisOne-Regular.ttf");

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
                font: font.clone(),
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
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Style::default()
            },
            ..ButtonBundle::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Play",
                    TextStyle {
                        font,
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
    mut interaction: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in interaction.iter_mut() {
        *color = if matches!(interaction, Interaction::None) {
            NORMAL_BUTTON_COLOR
        } else {
            ACTIVE_BUTTON_COLOR
        }
        .into();

        // Check if the button has been clicked
        if matches!(interaction, Interaction::Clicked) {
            state.set(AppState::LevelSelect).unwrap();
        }
    }
}
