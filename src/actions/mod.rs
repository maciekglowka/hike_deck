use bevy::prelude::*;
use std::collections::VecDeque;

use crate::states::GameState;

pub mod models;
mod systems;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActorQueue>()
            .init_resource::<PendingActions>()
            .add_event::<TickEvent>()
            .add_event::<NextActorEvent>()
            .add_event::<ActionsCompleteEvent>()
            .add_event::<InvalidPlayerActionEvent>()
            .configure_set(ActionSet::Planning.run_if(on_event::<NextActorEvent>()))
            .configure_set(ActionSet::Planning.before(ActionSet::Late))
            .add_system(systems::process_action_queue
                .run_if(on_event::<TickEvent>())
                .in_set(ActionSet::Late)
            )
            .add_system(systems::populate_actor_queue
                .in_schedule(OnExit(GameState::PlayerInput))
            )
            .add_systems(
                (systems::plan_walk, systems::plan_melee)
                .in_set(ActionSet::Planning)
            );
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum ActionSet {
    Planning,
    Late
}

pub trait Action: Send + Sync {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()>;
}

#[derive(Default, Resource)]
pub struct ActorQueue(pub VecDeque<Entity>);

#[derive(Default, Resource)]
pub struct PendingActions(pub Vec<Box<dyn Action>>);

pub struct TickEvent;
pub struct NextActorEvent;
pub struct ActionsCompleteEvent;
pub struct InvalidPlayerActionEvent;