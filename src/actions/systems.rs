use bevy::prelude::*;
use rand::prelude::*;

use crate::board::{
    CurrentBoard,
    components::Position
};
use crate::pieces::components::{Actor, Occupier, Walk};
use crate::player::Player;
use crate::vectors::{find_path, ORTHO_DIRECTIONS};

use super::{ActorQueue, ActionsCompleteEvent, InvalidPlayerActionEvent, NextActorEvent};
use super::models::WalkAction;

const PLAYER_ATTACK_SCORE: i32 = 100;
const MOVE_SCORE: i32 = 50;

pub fn process_action_queue(world: &mut World) {
    let Some(mut queue) = world.get_resource_mut::<ActorQueue>() else { return };
    let Some(entity) = queue.0.pop_front() else {
        world.send_event(ActionsCompleteEvent);
        return;
    };
    let Some(mut actor) = world.get_mut::<Actor>(entity) else { return };
    // clear the Actor vec
    let mut possible_actions = actor.0.drain(..).collect::<Vec<_>>();
    // highest score first
    possible_actions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut success = false;
    for action in possible_actions{
        if action.0.execute(world) {
            success = true;
            break;
        }
    }
    if !success && world.get::<Player>(entity).is_some() {
        world.send_event(InvalidPlayerActionEvent);
        return;
    }
    world.send_event(NextActorEvent);
}

pub fn populate_actor_queue(
    query: Query<Entity, (With<Actor>, Without<Player>)>,
    mut queue: ResMut<ActorQueue>
) {
    queue.0.extend(
        query.iter()
    );
}

pub fn plan_walk(
    mut query: Query<(&Position, &mut Actor), With<Walk>>,
    queue: Res<ActorQueue>,
    player_query: Query<&Position, With<Player>>,
    occupier_query: Query<&Position, With<Occupier>>,
    board: Res<CurrentBoard>,
) {
    let Some(entity) = queue.0.get(0) else { return };
    let Ok((position, mut actor)) = query.get_mut(*entity) else { return };
    let Ok(player_position) = player_query.get_single() else { return };
    // get all possible move targets
    let positions = ORTHO_DIRECTIONS.iter().map(|d| *d + position.v).collect::<Vec<_>>();
    // find possible path to the player
    let path_to_player = find_path(
        position.v,
        player_position.v,
        &board.tiles.keys().cloned().collect(),
        &occupier_query.iter().map(|p| p.v).collect()
    );
    let mut rng = thread_rng();
    let actions = positions.iter()
        .map(|v| {
            // randomize movement choices
            let mut d = rng.gen_range(-10..0);
            if let Some(path) = &path_to_player {
                // however prioritze a movement if it leads to the player
                if path.contains(v) { d = 5 }
            }
            (Box::new(WalkAction(*entity, *v)) as Box<dyn super::Action>, MOVE_SCORE + d)
        })
        .collect::<Vec<_>>();
    actor.0.extend(actions);
}