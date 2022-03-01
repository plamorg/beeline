use crate::{
    level_select::LevelSelectPlugin,
    menu::MenuPlugin,
    upgrade_select::{UpgradeButton, UpgradeSelectPlugin},
    AppState,
};
use bevy::{app::PluginGroupBuilder, prelude::*};

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.65, 0.8, 0.44);
pub const ACTIVE_BUTTON_COLOR: Color = Color::rgb(0.98, 0.82, 0.48);
pub const INACTIVE_BUTTON_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

struct UiPlugin;

#[derive(Clone)]
pub struct GameFont(Handle<Font>);

impl GameFont {
    pub fn get_handle(&self) -> Handle<Font> {
        self.0.clone()
    }
}

impl FromWorld for GameFont {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        Self(asset_server.load("FrancoisOne-Regular.ttf"))
    }
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameFont>()
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(manage_button_colors))
            .add_system_set(
                SystemSet::on_update(AppState::UpgradeSelect)
                    .with_system(manage_button_colors)
                    .with_system(manage_back_button),
            )
            .add_system_set(
                SystemSet::on_update(AppState::LevelSelect)
                    .with_system(manage_button_colors)
                    .with_system(manage_back_button),
            );
    }
}

pub struct UiPlugins;

impl PluginGroup for UiPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(LevelSelectPlugin)
            .add(MenuPlugin)
            .add(UpgradeSelectPlugin)
            .add(UiPlugin);
    }
}

#[derive(Component)]
struct BackButton;

pub fn spawn_back_button(commands: &mut Commands, font: Handle<Font>) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Percent(2.0),
                    left: Val::Percent(2.0),
                    ..Rect::default()
                },
                size: Size::new(Val::Px(100.0), Val::Px(40.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Style::default()
            },
            ..ButtonBundle::default()
        })
        .insert(BackButton)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Back",
                    TextStyle {
                        font,
                        font_size: 50.0,
                        color: Color::BLACK,
                    },
                    TextAlignment::default(),
                ),
                ..TextBundle::default()
            });
        });
}

fn manage_back_button(
    mut state: ResMut<State<AppState>>,
    interaction: Query<&Interaction, (Changed<Interaction>, With<BackButton>)>,
) {
    for interaction in interaction.iter() {
        if matches!(interaction, Interaction::Clicked) {
            // Go back menu state when back button is clicked
            state.set(AppState::Menu).unwrap();
        }
    }
}

fn manage_button_colors(
    mut interaction: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>, Without<UpgradeButton>),
    >,
) {
    for (interaction, mut color) in interaction.iter_mut() {
        *color = if matches!(interaction, Interaction::None) {
            NORMAL_BUTTON_COLOR
        } else {
            ACTIVE_BUTTON_COLOR
        }
        .into();
    }
}
