//! Level loading and related type defs

use crate::AppState;
use crate::level::bytereader::BytesResourceReader;
use crate::pipes::{InternalRouting, Pipe, PipeArchetypes, Slot};
use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, AsyncReadExt, LoadContext};
use bevy::ecs::error::info;
use bevy::prelude::*;
use bevy::tasks::ConditionalSendFuture;
use thiserror::Error;
use tiled::PropertyValue;

pub struct LevelPlugin;

#[derive(Asset, TypePath, Debug)]
pub struct Level {
    /// Machine-readable name of the level
    pub id: String,
    /// Display name of this level
    pub name: String,
    /// How many seconds until the input pipes activate?
    pub prepare_time: f32,
    pub data: LevelData,
}

#[derive(Debug)]
pub struct LevelData {
    pub size: UVec2,
    pub tiles: Vec<u32>,
}

/// Event for triggering the loading of a new level.
#[derive(Event, Debug)]
pub struct LoadNextLevel(pub String);

#[derive(Event, Debug)]
pub struct LevelLoaded(pub Handle<Level>);

#[derive(Resource, Debug)]
pub struct LevelInLoading(pub Handle<Level>);

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadNextLevel>()
            .add_event::<LevelLoaded>()
            .init_asset::<Level>()
            .init_asset_loader::<LevelLoader>()
            .add_systems(
                PreUpdate,
                (
                    begin_loading_level.run_if(on_event::<LoadNextLevel>),
                    wait_for_level_data.run_if(resource_exists::<LevelInLoading>),
                    cleanup.run_if(on_event::<LevelLoaded>),
                )
                    .chain(),
            );
    }
}

fn begin_loading_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<LoadNextLevel>,
) {
    if let Some(event) = events.read().last() {
        error!("Loading next level: {:?}", event);
        commands.insert_resource(LevelInLoading(asset_server.load(event.0.to_owned())));
    }
}

fn wait_for_level_data(
    mut commands: Commands,
    level_assets: Res<Assets<Level>>,
    level_in_loading: Res<LevelInLoading>,
    mut loaded_events: EventWriter<LevelLoaded>,
    pipe_archetypes: Res<PipeArchetypes>,
) {
    if let Some(level) = level_assets.get(&level_in_loading.0) {
        info!("Level asset loaded, spawning tiles");
        // spawn tiles
        let level_offset = Vec2::new(level.data.size.x as f32, level.data.size.y as f32) / 2.;

        for (index, tile) in level.data.tiles.iter().enumerate() {
            let row = ((index as f32) / level.data.size.x as f32).floor();
            let column = ((index as f32) % level.data.size.x as f32).floor();
            let tile_center = Vec2::new(row * 2., column * 2.) - level_offset;

            if let Some(pipe) = pipe_archetypes.get(tile) {
                commands.spawn((
                    pipe.clone(),
                    SceneRoot(pipe.model.clone()),
                    Transform::from_xyz(tile_center.x, 0., tile_center.y),
                ));
            }
        }

        // send event
        loaded_events.write(LevelLoaded(level_in_loading.0.clone()));
    }
}

fn cleanup(mut commands: Commands, mut app_state: ResMut<NextState<AppState>>) {
    commands.remove_resource::<LevelInLoading>();
    app_state.set(AppState::InGame);
}

#[derive(Default, Debug)]
struct LevelLoader;

#[derive(Debug, Error)]
enum LevelError {
    #[error("I/O error while loading level: {0}")]
    Io(#[from] std::io::Error),
    #[error("Tiled error while loading level: {0}")]
    Tiled(#[from] tiled::Error),
    #[error("Level is missing layer 0")]
    MissingLayer,
}

impl AssetLoader for LevelLoader {
    type Asset = Level;
    type Settings = ();
    type Error = LevelError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        use tiled::Loader;

        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let og_path = load_context.path().to_path_buf();

        let mut loader = Loader::with_reader(BytesResourceReader::new(&bytes, load_context));

        let map = loader.load_tmx_map(og_path.as_path())?;

        let tile_layer = map
            .get_layer(0)
            .ok_or(LevelError::MissingLayer)?
            .as_tile_layer()
            .ok_or(LevelError::MissingLayer)?;

        let mut tiles = Vec::with_capacity((map.width * map.height) as usize);

        for y in 0..map.height {
            for x in 0..map.width {
                let tile = tile_layer
                    .get_tile(x as i32, y as i32)
                    .expect("each cell should have a tile in the level");
                tiles.push(tile.tileset_index() as u32);
            }
        }

        let level = Level {
            id: og_path.to_string_lossy().to_string(),
            name: map
                .properties
                .get("level_name")
                .and_then(|v| match v {
                    PropertyValue::StringValue(s) => Some(s.to_string()),
                    _ => None,
                })
                .unwrap_or("Unnamed".into()),
            prepare_time: 0.0,
            data: LevelData {
                size: UVec2::new(map.width, map.width),
                tiles,
            },
        };

        Ok(level)
    }

    fn extensions(&self) -> &[&str] {
        &["tmx"]
    }
}

mod bytereader {
    use bevy::asset::LoadContext;
    use std::{
        io::{Cursor, Error as IoError, ErrorKind, Read},
        path::Path,
        sync::Arc,
    };

    pub(crate) struct BytesResourceReader<'a, 'b> {
        bytes: Arc<[u8]>,
        context: &'a mut LoadContext<'b>,
    }
    impl<'a, 'b> BytesResourceReader<'a, 'b> {
        pub(crate) fn new(bytes: &'a [u8], context: &'a mut LoadContext<'b>) -> Self {
            Self {
                bytes: Arc::from(bytes),
                context,
            }
        }
    }

    impl<'a> tiled::ResourceReader for BytesResourceReader<'a, '_> {
        type Resource = Box<dyn Read + 'a>;
        type Error = IoError;

        fn read_from(&mut self, path: &Path) -> std::result::Result<Self::Resource, Self::Error> {
            if let Some(extension) = path.extension() {
                if extension == "tsx" {
                    let future = self.context.read_asset_bytes(path.to_path_buf());
                    let data = futures_lite::future::block_on(future)
                        .map_err(|err| IoError::new(ErrorKind::NotFound, err))?;
                    return Ok(Box::new(Cursor::new(data)));
                }
            }
            Ok(Box::new(Cursor::new(self.bytes.clone())))
        }
    }
}
