use bevy::prelude::*;

use crate::board::components::Position;
use crate::states::MainState;
use crate::vectors::Vector2Int;

pub mod components;

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_npcs.in_schedule(OnEnter(MainState::Game)));
    }
}

pub fn spawn_npcs(
    mut commands: Commands
) {
    commands.spawn((
        components::Actor::default(),
        components::Piece { kind: "NPC".to_string() },
        Position { v: Vector2Int::new(3, 5) },
        components::Walk
    ));
    commands.spawn((
        components::Actor::default(),
        components::Piece { kind: "NPC".to_string() },
        Position { v: Vector2Int::new(5, 5) },
        components::Walk
    ));
}