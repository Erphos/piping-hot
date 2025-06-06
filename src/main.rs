mod assets;
mod game;
mod level;
mod menu;
mod pipes;

use crate::assets::AssetsPlugin;
use crate::game::PipeGamePlugin;
use crate::level::LevelPlugin;
use crate::menu::MenuPlugin;
use crate::pipes::PipePlugin;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Wasm builds will check for meta files (that don't exist) if this isn't set.
            // This causes errors and even panics in web builds on itch.
            // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_plugins(MeshPickingPlugin)
        .init_state::<AppState>()
        .add_plugins((
            AssetsPlugin,
            MenuPlugin,
            LevelPlugin,
            PipePlugin,
            PipeGamePlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum AppState {
    #[default]
    LoadingAssets,
    MainMenu,
    LoadingLevel,
    InGame,
}

fn setup(mut commands: Commands) {
    // Default UI camera
    commands.spawn((
        Camera2d,
        Camera {
            order: 99,
            ..default()
        },
    ));
}
