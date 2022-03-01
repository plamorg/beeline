use benimator::{AnimationMode, Play, SpriteSheetAnimation};
use bevy::prelude::*;
use std::{path::PathBuf, time::Duration};

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
    pub fn new(
        animations: &mut ResMut<Assets<SpriteSheetAnimation>>,
        textures: &mut ResMut<Assets<TextureAtlas>>,
        asset_server: &Res<AssetServer>,
        data: AnimatedSpriteData,
    ) -> Self {
        let animation_handle = animations.add({
            let sheet = SpriteSheetAnimation::from_range(0..=(data.frames - 1), data.delay);
            match data.mode {
                AnimationMode::Once => sheet.once(),
                AnimationMode::Repeat => sheet.repeat(),
                AnimationMode::PingPong => sheet.ping_pong(),
                _ => unimplemented!(),
            }
        });

        let sprite_sheet_bundle = SpriteSheetBundle {
            texture_atlas: textures.add(TextureAtlas::from_grid(
                asset_server.load(data.path),
                data.size,
                data.frames,
                1,
            )),
            transform: data.transform,
            ..SpriteSheetBundle::default()
        };

        Self {
            animation_handle,
            play: Play,
            sprite_sheet_bundle,
        }
    }
}

pub struct AnimatedSpriteData {
    pub path: PathBuf,
    pub frames: usize,
    pub size: Vec2,
    pub transform: Transform,
    pub delay: Duration,
    pub mode: AnimationMode,
}

impl Default for AnimatedSpriteData {
    fn default() -> Self {
        Self {
            path: PathBuf::default(),
            frames: usize::default(),
            size: Vec2::default(),
            transform: Transform::default(),
            delay: Duration::from_millis(100),
            mode: AnimationMode::Repeat,
        }
    }
}
