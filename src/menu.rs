//! Game menu

use crate::AppState;
use crate::assets::UiAssets;
use crate::level::LoadNextLevel;
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
            .add_systems(
                Update,
                (update_button_color, menu_action).run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(OnExit(AppState::MainMenu), teardown_main_menu);
    }
}

/// Marker component for menu items that should be automatically removed when the app state changes.
#[derive(Component, Debug)]
struct MenuItem;

#[derive(Component, Debug)]
enum MenuAction {
    StartGame,
    Quit,
}

fn setup_main_menu(mut commands: Commands, assets: Res<UiAssets>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            MenuItem,
        ))
        .with_children(|cmd| {
            cmd.spawn(button("New Game", &assets))
                .insert(MenuAction::StartGame);
            cmd.spawn(button("Quit", &assets)).insert(MenuAction::Quit);
        });
}

fn button(text: &str, assets: &UiAssets) -> impl Bundle + use<> {
    (
        Button,
        Node {
            width: Val::Px(250.0),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor(Color::BLACK),
        BorderRadius::MAX,
        // TODO: color palette asset
        BackgroundColor(Color::srgb_u8(76, 146, 212)),
        children![(
            Text::new(text),
            TextFont {
                font: assets.button_font.clone(),
                font_size: 33.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextShadow::default(),
        )],
    )
}

fn update_button_color(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgb_u8(200, 200, 200).into();
                border_color.0 = Color::srgb_u8(200, 41, 13);
            }
            Interaction::Hovered => {
                *color = Color::srgb_u8(176, 146, 112).into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = Color::srgb_u8(76, 146, 212).into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
fn menu_action(
    interaction_query: Query<(&Interaction, &MenuAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut app_state: ResMut<NextState<AppState>>,
    mut load_level: EventWriter<LoadNextLevel>,
) {
    for (interaction, menu_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_action {
                MenuAction::StartGame => {
                    load_level.write(LoadNextLevel("levels/map.tmx".into()));
                    app_state.set(AppState::LoadingLevel);
                }
                MenuAction::Quit => {
                    app_exit_events.write(AppExit::Success);
                }
            }
        }
    }
}

fn teardown_main_menu(mut commands: Commands, menu_items: Query<Entity, With<MenuItem>>) {
    for entity in &menu_items {
        commands.entity(entity).despawn();
    }
}
