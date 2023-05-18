use bevy::prelude::*;
use std::collections::VecDeque;


use crate::actions::ActorQueue;
use crate::board::components::Position;
use crate::pieces::components::{Actor, Health, Occupier, Piece};
use crate::states::MainState;
use crate::vectors::Vector2Int;


pub mod cards;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DeckEvent>()
            .add_event::<PlayerActionEvent>()
            .add_system(spawn_player.in_schedule(OnEnter(MainState::Game)))
            .add_system(dispatch_card.run_if(on_event::<DeckEvent>()))
            .add_system(select_card.run_if(on_event::<DeckEvent>()));
    }
}

pub enum DeckEventKind {
    // emit from the input system to mark active card in the deck
    SelectCard(Entity),
    // emit from the input system to use the card with optional target coordinate
    UseCard(Option<Vector2Int>)
}

pub struct DeckEvent(pub DeckEventKind);

// mark that the player is ready to execute game action
pub struct PlayerActionEvent;

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

pub fn select_card(
    mut ev_deck: EventReader<DeckEvent>,
    mut deck: ResMut<Deck>
) {
    for ev in ev_deck.iter() {
        if let DeckEvent(DeckEventKind::SelectCard(entity)) = ev {
            deck.current_card = Some(*entity);
        }
    }
}

pub fn dispatch_card(
    mut ev_deck: EventReader<DeckEvent>,
    mut ev_action: EventWriter<PlayerActionEvent>,
    deck: Res<Deck>,
    mut player_query: Query<(Entity, &mut Actor), With<Player>>,
    card_query: Query<&cards::CardHolder>,
    mut queue: ResMut<ActorQueue>
) {
    for ev in ev_deck.iter() {
        if let DeckEvent(DeckEventKind::UseCard(v)) = ev {
            let Ok((entity, mut actor)) = player_query.get_single_mut() else { return };
            let Some(card_entity) = deck.current_card else { return };
            let Ok(card) = card_query.get(card_entity) else { continue };
            let Some(action) = card.0.get_action(entity, *v) else { continue };
            
            // action score does not matter for the player
            actor.0 = vec![(action, 0)];

            // the player moves first, so start with a single element queue
            queue.0 = VecDeque::from([entity]);
            ev_action.send(PlayerActionEvent);
        }
    }
}