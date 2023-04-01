use bevy::prelude::*;

use crate::actions::{TickEvent, ActionsCompleteEvent, InvalidPlayerActionEvent};
use crate::graphics::GraphicsWaitEvent;
use crate::input::PlayerInputReadyEvent;
use crate::states::{GameState, MainState};

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(game_start.in_schedule(OnEnter(MainState::Game)))
            .add_system(game_end.in_schedule(OnExit(MainState::Game)))
            .add_system(turn_update_start.run_if(on_event::<PlayerInputReadyEvent>()))
            .add_system(turn_update_end.run_if(on_event::<ActionsCompleteEvent>()))
            .add_system(turn_update_cancel.run_if(on_event::<InvalidPlayerActionEvent>()))
            .add_system(tick.in_set(OnUpdate(GameState::TurnUpdate)));
    }
}

fn game_start(
    mut next_state: ResMut<NextState<GameState>>
) {
    next_state.set(GameState::PlayerInput);
}

fn game_end(
    mut next_state: ResMut<NextState<GameState>>
) {
    next_state.set(GameState::None);
}

fn turn_update_start(
    mut next_state: ResMut<NextState<GameState>>,
    mut ev_tick: EventWriter<TickEvent>
) {
    next_state.set(GameState::TurnUpdate);
    ev_tick.send(TickEvent);
}

fn tick(
    mut ev_wait: EventReader<GraphicsWaitEvent>,
    mut ev_tick: EventWriter<TickEvent>
) {
    if ev_wait.iter().len() == 0 {
        ev_tick.send(TickEvent);
    }
}

fn turn_update_end(
    mut next_state: ResMut<NextState<GameState>>
) {
    next_state.set(GameState::PlayerInput);
}

fn turn_update_cancel(
    mut next_state: ResMut<NextState<GameState>>
) {
    next_state.set(GameState::PlayerInput);
}