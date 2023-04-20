use bevy::prelude::*;


use crate::board::components::Position;
use crate::pieces::components::{Actor, Health, Occupier, Piece};
use crate::states::MainState;
use crate::vectors::Vector2Int;


mod cards;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(MainState::Game)));
    }
}

#[derive(Default, Resource)]
pub struct Deck {
    pub cards: Vec<Entity>,
    pub current_card: Option<Entity>
}

#[derive(Component)]
pub struct Player;

fn spawn_player(
    mut commands: Commands
) {
    let walk_card = commands.spawn(
            cards::CardHolder(Box::new(cards::WalkCard))
        ).id();
    let melee_card = commands.spawn(
            cards::CardHolder(Box::new(cards::MeleeCard(1)))
        ).id();

    commands.insert_resource(
        Deck { cards: vec![walk_card, melee_card], ..Default::default() }
    );

    commands.spawn((
        Actor::default(),
        Health { value: 3 },
        Occupier,
        Player,
        Piece { kind: "Player".to_string() },
        Position { v: Vector2Int::new(0, 0) }
    ));
}