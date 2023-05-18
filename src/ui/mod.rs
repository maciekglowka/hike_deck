use bevy::prelude::*;
use std::collections::HashMap;

use crate::states::GameState;

mod assets;
mod deck;
mod helpers;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ReloadUiEvent>()
            .add_startup_system(assets::load_assets)
            .add_system(helpers::button_click_animation)
            .add_system(
                player_input_start.in_schedule(OnEnter(GameState::PlayerInput))
            )
            .add_system(
                deck::draw_deck.run_if(on_event::<ReloadUiEvent>())
            )
            .add_system(
                deck::card_click.in_set(OnUpdate(GameState::PlayerInput))
            );
    }
}

pub struct ReloadUiEvent;

fn player_input_start(
    mut ev_ui: EventWriter<ReloadUiEvent>
) {
    ev_ui.send(ReloadUiEvent);
}

#[derive(Resource)]
pub struct UiAssets {
    pub font: Handle<Font>,
    pub textures: HashMap<&'static str, Handle<Image>>
}