use bevy::prelude::*;
use std::collections::HashMap;

use super::CurrentBoard;
use super::dungeon::{Area, Dungeon};
use super::components::{Position, Tile};

pub fn spawn_map(
    mut commands: Commands,
    mut current: ResMut<CurrentBoard>
) {
    let mut dungeon = Dungeon::new(2);
    for _ in 0..4 {
        dungeon.add_area(Area::new())
    }
    dungeon.generate();

    current.tiles = HashMap::new();
    for v in dungeon.to_tiles() {
        let tile = commands.spawn((
                Position { v },
                Tile
            ))
            .id();
        current.tiles.insert(v, tile);
    }
}