use bevy::prelude::*;

use crate::actions::Action;

#[derive(Component, Default)]
pub struct Actor(pub Option<Box<dyn Action>>);

#[derive(Component)]
pub struct Piece {
    pub kind: String
}

#[derive(Component)]
// movement behaviour for non-player pieces
pub struct Walk;