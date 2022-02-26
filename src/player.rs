use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player);
    }
}

#[derive(Component)]
struct Player;

impl Player {
    const SIZE: f32 = 16.0;
}

fn spawn_player(mut commands: Commands) {
    // Define player size
    let size = Vec2::splat(Player::SIZE);

    // Spawn player
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW,
                custom_size: Some(size),
                ..Sprite::default()
            },
            transform: Transform::identity(),
            ..SpriteBundle::default()
        })
        .insert(Player);
}
