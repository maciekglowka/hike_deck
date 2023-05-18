use bevy::prelude::*;

use crate::vectors::Vector2Int;


#[derive(Component)]
pub struct Position {
    pub v: Vector2Int
}

#[derive(Component)]
pub struct Tile;