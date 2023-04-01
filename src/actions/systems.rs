use bevy::prelude::*;

use crate::pieces::components::Actor;
use crate::player::Player;

use super::{ActorQueue, ActionsCompleteEvent, InvalidPlayerActionEvent, NextActorEvent};

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