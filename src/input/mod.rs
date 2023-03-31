use bevy::prelude::*;

use crate::board::components::Position;
use crate::player::Player;
use crate::vectors::Vector2Int;


pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_position);
    }
}

const DIR_KEY_MAPPING: [(KeyCode, Vector2Int); 4] = [
    (KeyCode::W, Vector2Int::UP), (KeyCode::S, Vector2Int::DOWN),
    (KeyCode::A, Vector2Int::LEFT), (KeyCode::D, Vector2Int::RIGHT),
];

fn player_position(
    keys: ResMut<Input<KeyCode>>,
    mut player_query: Query<&mut Position, With<Player>>,
) {
    let Ok(mut position) = player_query.get_single_mut() else { return };
    for (key, dir) in DIR_KEY_MAPPING {
        if !keys.just_pressed(key) { continue; }
        position.v += dir;
    }
}
