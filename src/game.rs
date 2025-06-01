//! Game logic
use bevy::prelude::*;

use crate::AppState;

pub struct PipeGamePlugin;

impl Plugin for PipeGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<PipeGameState>();
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
