use bevy::prelude::*;

mod assets;
mod board;
mod globals;
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
        .add_plugin(assets::AssetPlugin)
        .add_plugin(board::BoardPlugin)
        .run()
}
