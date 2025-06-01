//! Game menu

use crate::AppState;
use crate::assets::UiAssets;
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
            .add_systems(Update, handle_buttons.run_if(in_state(AppState::MainMenu)))
            .add_systems(OnExit(AppState::MainMenu), teardown_main_menu);
    }
}

/// Marker component for menu items that should be automatically removed when the app state changes.
#[derive(Component, Debug)]
struct MenuItem;

fn setup_main_menu(mut commands: Commands, assets: Res<UiAssets>) {
    commands.spawn(button("New Game", &assets));
}

fn button(text: &str, assets: &UiAssets) -> impl Bundle + use<> {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
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
            )]
        )],
    )
}

fn handle_buttons(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
}

fn teardown_main_menu(mut commands: Commands, menu_items: Query<Entity, With<MenuItem>>) {
    for entity in &menu_items {
        commands.entity(entity).despawn();
    }
}
