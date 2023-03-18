use bevy::prelude::*;

use crate::board::components::{Position, Tile};
use super::{GraphicsAssets, TILE_SIZE};

pub fn spawn_tile_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position), Added<Tile>>,
    assets: Res<GraphicsAssets>
) {
    for (entity, position) in query.iter() {
        let mut sprite = TextureAtlasSprite::new(177);
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
        sprite.color = Color::OLIVE;
        let v = Vec3::new(
            TILE_SIZE * position.v.x as f32,
            TILE_SIZE * position.v.y as f32,
            0.
        );
        commands.entity(entity)
            .insert(
                SpriteSheetBundle {
                    sprite,
                    texture_atlas: assets.sprite_texture.clone(),
                    transform: Transform::from_translation(v),
                    ..Default::default()
                }
            );
    }
}