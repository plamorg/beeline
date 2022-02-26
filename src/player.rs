use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player).add_system(move_player);
    }
}

#[derive(Component)]
pub struct Player;

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

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut transform: Query<&mut Transform, With<Player>>,
) {
    let mut transform = transform.single_mut();

    for key in keyboard_input.get_just_pressed() {
        // Move player translation based on keyboard input
        match key {
            KeyCode::Left | KeyCode::A => transform.translation.x -= Player::SIZE,
            KeyCode::Right | KeyCode::D => transform.translation.x += Player::SIZE,
            KeyCode::Up | KeyCode::W => transform.translation.y += Player::SIZE,
            KeyCode::Down | KeyCode::S => transform.translation.y -= Player::SIZE,
            _ => {}
        }
    }
}
