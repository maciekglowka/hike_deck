use bevy::prelude::*;


use crate::board::components::Position;
use crate::pieces::components::{Actor, Piece};
use crate::states::MainState;
use crate::vectors::Vector2Int;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(MainState::Game)));
    }
}


#[derive(Component)]
pub struct Player;

fn spawn_player(
    mut commands: Commands
) {
    commands.spawn((
        Actor::default(),
        Player,
        Piece { kind: "Player".to_string() },
        Position { v: Vector2Int::new(0, 0) }
    ));
}