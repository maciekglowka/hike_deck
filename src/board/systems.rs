use bevy::prelude::*;
use std::collections::HashMap;

use super::CurrentBoard;
use super::dungeon::{Area, Dungeon, tunneler, room};
use super::components::{Position, Tile};

pub fn spawn_map(
    mut commands: Commands,
    mut current: ResMut<CurrentBoard>
) {
    let mut dungeon = Dungeon::new(2);
    for idx in 0..4 {
        let tun = match idx % 2 {
            0 => Box::new(tunneler::LShapeTunneler) as Box<dyn tunneler::Tunneler>,
            _ => Box::new(tunneler::RandomTunneler) as Box<dyn tunneler::Tunneler>
        };
        let gen = Box::new(room::BubbleGenerator {
            room_count: (3, 5),
            room_size: (4, 8),
            room_padding: Some(2),
            extra_connection_chance: 0.25
        }) as Box<dyn room::RoomGenerator>;
        dungeon.add_area(Area::new(tun, gen))
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