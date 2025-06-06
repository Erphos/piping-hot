//! Game logic
use bevy::prelude::*;

use crate::AppState;

pub struct PipeGamePlugin;

impl Plugin for PipeGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<PipeGameState>()
            .add_systems(OnEnter(AppState::InGame), setup_game_scene);
    }
}

#[derive(SubStates, Debug, Default, Clone, Eq, PartialEq, Hash)]
#[source(AppState = AppState::InGame)]
pub enum PipeGameState {
    #[default]
    Playing,
    Simulating,
    LevelWon,
    LevelFailed,
}

fn setup_game_scene(mut commands: Commands) {
    info!("Setting up game scene");
    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 0,
            ..default()
        },
        Transform::from_xyz(0., 10., 10.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
