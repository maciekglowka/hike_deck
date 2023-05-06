use bevy::prelude::*;
use std::collections::HashMap;

use super::UiAssets;

const TEXTURES: [&str; 1] = ["card"];

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut asset_list: ResMut<crate::assets::AssetList>
) {
    // font via http://www.dsg4.com/
    let font = asset_server.load("ui/04B_03.ttf");
    asset_list.0.push(font.clone_untyped());

    let mut textures = HashMap::new();
    for name in TEXTURES {
        let handle = asset_server.load(format!("ui/{}.png", name));
        asset_list.0.push(handle.clone_untyped());
        textures.insert(name, handle);
    }

    commands.insert_resource(UiAssets { textures, font });
}