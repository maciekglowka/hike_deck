use bevy::prelude::*;

mod actions;
mod assets;
mod board;
mod camera;
mod globals;
mod graphics;
mod input;
mod manager;
mod pieces;
mod player;
mod states;
mod vectors;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (
                            globals::WINDOW_WIDTH,
                            globals::WINDOW_HEIGHT
                        ).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            ).set(
                ImagePlugin::default_nearest()
            )
        )
        .insert_resource(Msaa::Off)
        .add_state::<states::MainState>()
        .add_state::<states::GameState>()
        .add_plugin(actions::ActionsPlugin)
        .add_plugin(assets::AssetPlugin)
        .add_plugin(board::BoardPlugin)
        .add_plugin(graphics::GraphicsPlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(manager::ManagerPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_startup_system(camera::setup)
        .run()
}
