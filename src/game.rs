//! Game logic
use crate::AppState;
use crate::level::{CurrentLevel, Level};
use bevy::prelude::*;
use serde::Deserialize;

pub struct PipeGamePlugin;

impl Plugin for PipeGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<PipeGameState>()
            .add_systems(OnEnter(AppState::InGame), setup_game_scene)
            .add_systems(OnExit(AppState::InGame), cleanup)
            .add_systems(
                Update,
                (warmup_timer).run_if(in_state(PipeGameState::Warmup)),
            );
    }
}

#[derive(SubStates, Debug, Default, Clone, Eq, PartialEq, Hash)]
#[source(AppState = AppState::InGame)]
pub enum PipeGameState {
    #[default]
    Warmup,
    Prepare,
    Flowing,
    LevelWon,
    LevelFailed,
}

/// Marker for game state entities for automatic cleanup.
#[derive(Component, Debug)]
struct GameEntity;

fn setup_game_scene(
    mut commands: Commands,
    levels: Res<Assets<Level>>,
    current_level: Res<CurrentLevel>,
) {
    info!("Setting up game scene");
    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 0,
            ..default()
        },
        Transform::from_xyz(0., 15., 5.).looking_at(Vec3::ZERO, Vec3::Y),
        GameEntity,
    ));

    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(4., 10., 8.).looking_at(Vec3::ZERO, Vec3::Y),
        GameEntity,
    ));

    // set warmup timer (grace period before becomes interactive)
    commands.insert_resource(WarmupTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}

fn cleanup(mut commands: Commands, entities: Query<Entity, With<GameEntity>>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }

    commands.remove_resource::<WarmupTimer>();
}

#[derive(Resource, Debug)]
struct WarmupTimer(Timer);

fn warmup_timer(
    mut timer: ResMut<WarmupTimer>,
    mut game_state: ResMut<NextState<PipeGameState>>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        info!("Warmup is finished");
        game_state.set(PipeGameState::Prepare);
    }
}
