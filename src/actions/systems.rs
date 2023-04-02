use bevy::prelude::*;
use rand::prelude::*;

use crate::board::components::Position;
use crate::pieces::components::{Actor, Walk};
use crate::player::Player;
use crate::vectors::ORTHO_DIRECTIONS;

use super::{ActorQueue, ActionsCompleteEvent, InvalidPlayerActionEvent, NextActorEvent};
use super::models::WalkAction;

pub fn process_action_queue(world: &mut World) {
    let Some(mut queue) = world.get_resource_mut::<ActorQueue>() else { return };
    let Some(entity) = queue.0.pop_front() else {
        world.send_event(ActionsCompleteEvent);
        return;
    };
    let Some(mut actor) = world.get_mut::<Actor>(entity) else { return };
    let Some(action) = actor.0.take() else { return };

    if !action.execute(world) && world.get::<Player>(entity).is_some() {
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
    queue: Res<ActorQueue>
) {
    let Some(entity) = queue.0.get(0) else { return };
    let Ok((position, mut actor)) = query.get_mut(*entity) else { return };
    let mut rng = thread_rng();
    let dir = ORTHO_DIRECTIONS.choose(&mut rng).unwrap();
    actor.0 = Some(Box::new(WalkAction(*entity, position.v + *dir)));
}