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
    spawn_test_npc(&mut commands, Vector2Int::new(3, 5));
    spawn_test_npc(&mut commands, Vector2Int::new(5, 5));
}

fn spawn_test_npc(commands: &mut Commands, v: Vector2Int) {
    commands.spawn((
        components::Actor::default(),
        components::Health { value: 1 },
        components::Piece { kind: "NPC".to_string() },
        components::Melee { damage: 1 },
        components::Occupier,
        Position { v },
        components::Walk
    ));
}