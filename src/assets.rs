//! Asset loading

use crate::AppState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource, Debug)]
pub struct ModelAssets {
    #[asset(path = "models/pipe.glb")]
    pub pipe: Handle<Gltf>,
}

#[derive(AssetCollection, Resource, Debug)]
pub struct UiAssets {
    #[asset(path = "fonts/Nunito-Black.ttf")]
    pub button_font: Handle<Font>,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::LoadingAssets)
                .continue_to_state(AppState::MainMenu)
                .load_collection::<ModelAssets>()
                .load_collection::<UiAssets>(),
        );
    }
}
