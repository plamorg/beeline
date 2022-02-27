use benimator::{Play, SpriteSheetAnimation};
use bevy::{asset::AssetPath, prelude::*};
use std::time::Duration;

pub fn polar_to_cartesian(angle: f32, length: f32) -> Vec2 {
    Vec2::new(length * angle.cos(), length * angle.sin())
}

#[derive(Bundle)]
pub struct AnimatedSprite {
    animation_handle: Handle<SpriteSheetAnimation>,
    play: Play,

    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

impl<'a> AnimatedSprite {
    pub fn new<P: Into<AssetPath<'a>>>(
        animations: &mut ResMut<Assets<SpriteSheetAnimation>>,
        textures: &mut ResMut<Assets<TextureAtlas>>,
        asset_server: &Res<AssetServer>,
        path: P,
        frames: usize,
        size: Vec2,
        position: Vec2,
    ) -> Self {
        let animation_handle = animations.add(SpriteSheetAnimation::from_range(
            0..=(frames - 1),
            Duration::from_millis(100),
        ));

        let sprite_sheet_bundle = SpriteSheetBundle {
            texture_atlas: textures.add(TextureAtlas::from_grid(
                asset_server.load(path),
                size,
                8,
                1,
            )),
            transform: Transform::from_translation(position.extend(0.0)),
            ..SpriteSheetBundle::default()
        };

        Self {
            animation_handle,
            play: Play,
            sprite_sheet_bundle,
        }
    }
}
