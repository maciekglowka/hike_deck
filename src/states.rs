use bevy::prelude::*;

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq)]
pub enum MainState {
    #[default]
    LoadAssets,
    Game
}

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq)]
pub enum GameState {
    #[default]
    None,
    PlayerInput,
    TurnUpdate
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum TurnSet {
    Logic,
    Animation,
    Tick
}