//! Level select

use crate::AppState;
use crate::assets::UiAssets;
use crate::level::LoadNextLevel;
use crate::menu::{self, MenuItem};
use bevy::prelude::*;



pub struct LevelSelectPlugin;

impl Plugin for LevelSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::LevelSelect), setup_level_select)
            .add_systems(
                Update,
                (menu::update_button_color, ls_action).run_if(in_state(AppState::LevelSelect)),
            )
            .add_systems(OnExit(AppState::LevelSelect), menu::teardown_menu);
    }
}

#[derive(Component, Debug)]
enum MenuAction {
    PlayLevel(String),
    Back
}


fn setup_level_select(mut commands: Commands, assets: Res<UiAssets>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                column_gap: Val::Px(10.0),
                row_gap: Val::Px(10.0),
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::auto(6),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                justify_items: JustifyItems::Center,
                align_content: AlignContent::Center,
                ..default()
            },
            MenuItem,
        ))
        .with_children(|cmd| {
            for i in 1..=3 {
                for j in 1..=6 {
                    let lname = format!("{}-{}", i, j);
                    cmd.spawn(menu::button_small(&lname, &assets))
                        .insert(MenuAction::PlayLevel(lname));
                }
            }
        });
}

fn ls_action(
    interaction_query: Query<(&Interaction, &MenuAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut app_state: ResMut<NextState<AppState>>,
    mut load_level: EventWriter<LoadNextLevel>,
) {
    for (interaction, menu_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_action {
                MenuAction::PlayLevel(i) => {
                    load_level.write(LoadNextLevel(format!("levels/{i}.tmx")));
                    app_state.set(AppState::LoadingLevel);
                }
                MenuAction::Back => {
                    app_exit_events.write(AppExit::Success);
                }
            }
        }
    }
}
