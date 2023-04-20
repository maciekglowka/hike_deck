use bevy::prelude::*;

use crate::board::components::Position;
use crate::player::{Player, Deck, DeckEvent, DeckEventKind};
use crate::states::GameState;
use crate::vectors::Vector2Int;


pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_input.in_set(OnUpdate(GameState::PlayerInput)));
    }
}

const DIR_KEY_MAPPING: [(KeyCode, Vector2Int); 4] = [
    (KeyCode::W, Vector2Int::UP), (KeyCode::S, Vector2Int::DOWN),
    (KeyCode::A, Vector2Int::LEFT), (KeyCode::D, Vector2Int::RIGHT),
];

fn player_input(
    keys: ResMut<Input<KeyCode>>,
    mut player_query: Query<&Position, With<Player>>,
    deck: Res<Deck>,
    mut ev_deck: EventWriter<DeckEvent>,
) {
    let Ok(position) = player_query.get_single_mut() else { return };
    for (key, dir) in DIR_KEY_MAPPING {
        if !keys.just_pressed(key) { continue; }
        ev_deck.send(DeckEvent(
            DeckEventKind::UseCard(Some(position.v + dir))
        ));
    }

    // use this to temporarily switch between our only two cards
    if keys.just_pressed(KeyCode::Key1) {
        if let Some(entity) = deck.cards.get(0) {
            ev_deck.send(DeckEvent(
                DeckEventKind::SelectCard(*entity)
            ));
        }
    }
    if keys.just_pressed(KeyCode::Key2) {
        if let Some(entity) = deck.cards.get(1) {
            ev_deck.send(DeckEvent(
                DeckEventKind::SelectCard(*entity)
            ));
        }
    }
}
