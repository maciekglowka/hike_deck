use bevy::prelude::*;

use crate::board::components::Position;

pub const TILE_SIZE: f32 = 32.;
pub const TILE_Z: f32 = 0.;
pub const PIECE_Z: f32 = 10.;
pub const PIECE_SPEED: f32 = 15.;
pub const POSITION_TOLERANCE: f32 = 0.1;

mod assets;
mod pieces;
mod tiles;

#[derive(Resource)]
pub struct GraphicsAssets {
    pub sprite_texture: Handle<TextureAtlas>
}

pub struct GraphicsWaitEvent;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GraphicsWaitEvent>()
            .add_startup_system(assets::load_assets)
            .add_system(pieces::spawn_piece_renderer)
            .add_system(pieces::update_piece_position)
            .add_system(tiles::spawn_tile_renderer);
    }
}

fn get_world_position(
    position: &Position,
    z: f32
) -> Vec3 {
    Vec3::new(
        TILE_SIZE * position.v.x as f32,
        TILE_SIZE * position.v.y as f32,
        z
    )
}