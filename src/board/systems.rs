use bevy::prelude::*;
use std::collections::HashMap;

use crate::vectors::Vector2Int;

use super::CurrentBoard;
use super::components::{Position, Tile};

pub fn spawn_map(
    mut commands: Commands,
    mut current: ResMut<CurrentBoard>
) {
    current.tiles = HashMap::new();
    for x in 0..8 {
        for y in 0..8 {
            let v = Vector2Int::new(x, y);
            let tile = commands.spawn((
                    Position { v },
                    Tile
                ))
                .id();
            current.tiles.insert(v, tile);
        }
    }
}