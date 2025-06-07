//! Pipe definitions

use bevy::platform::collections::HashMap;
use bevy::prelude::*;

type SlotId = u8;

type FluidId = String;

pub struct PipePlugin;

#[derive(Resource, Debug, DerefMut, Deref)]
pub struct PipeArchetypes(HashMap<u32, Pipe>);

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_pipe_archetypes);
    }
}

fn initialize_pipe_archetypes(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut pipes = PipeArchetypes(HashMap::new());

    // Input
    pipes.insert(
        16,
        Pipe {
            source: Some("water".into()),
            sink: None,
            slots: [Slot::Output, Slot::None, Slot::None, Slot::None],
            progress: 0.0,
            progress_rate: 1.0,
            internal_routing: vec![],
            model: asset_server.load(GltfAssetLabel::Scene(6).from_asset("models/pipe.glb")),
            locked: false,
        },
    );

    // Output
    pipes.insert(
        32,
        Pipe {
            source: None,
            sink: Some("water".into()),
            slots: [Slot::Input, Slot::None, Slot::None, Slot::None],
            progress: 0.0,
            progress_rate: 1.0,
            internal_routing: vec![],
            model: asset_server.load(GltfAssetLabel::Scene(6).from_asset("models/pipe.glb")),
            locked: false,
        },
    );

    // Straight pipe
    pipes.insert(
        0,
        Pipe {
            source: None,
            sink: None,
            slots: [
                Slot::Bidirectional,
                Slot::None,
                Slot::Bidirectional,
                Slot::None,
            ],
            progress: 0.0,
            progress_rate: 1.0,
            internal_routing: vec![InternalRouting::passthrough(0, 2)],
            model: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/pipe.glb")),
            locked: false,
        },
    );

    // Curved pipe
    pipes.insert(
        1,
        Pipe {
            source: None,
            sink: None,
            slots: [
                Slot::Bidirectional,
                Slot::Bidirectional,
                Slot::None,
                Slot::None,
            ],
            progress: 0.0,
            progress_rate: 1.0,
            internal_routing: vec![InternalRouting::passthrough(0, 1)],
            model: asset_server.load(GltfAssetLabel::Scene(1).from_asset("models/pipe.glb")),
            locked: false,
        },
    );

    // Cork
    pipes.insert(
        2,
        Pipe {
            source: None,
            sink: None,
            slots: [Slot::Bidirectional, Slot::None, Slot::None, Slot::None],
            progress: 0.0,
            progress_rate: 1.0,
            internal_routing: vec![InternalRouting::passthrough(0, 5)],
            model: asset_server.load(GltfAssetLabel::Scene(2).from_asset("models/pipe.glb")),
            locked: false,
        },
    );

    // T pipe
    pipes.insert(
        3,
        Pipe {
            source: None,
            sink: None,
            slots: [
                Slot::Bidirectional,
                Slot::Bidirectional,
                Slot::Bidirectional,
                Slot::None,
            ],
            progress: 0.0,
            progress_rate: 1.0,
            internal_routing: vec![
                InternalRouting::mix(0, 5),
                InternalRouting::mix(1, 5),
                InternalRouting::mix(2, 5),
                InternalRouting::passthrough(5, 0),
                InternalRouting::passthrough(5, 1),
                InternalRouting::passthrough(5, 2),
            ],
            model: asset_server.load(GltfAssetLabel::Scene(3).from_asset("models/pipe.glb")),
            locked: false,
        },
    );

    commands.insert_resource(pipes);
}

pub struct Fluid {
    pub id: FluidId,
    pub material: Handle<StandardMaterial>,
}

#[derive(Resource)]
pub struct Fluids(HashMap<FluidId, Fluid>);

#[derive(Debug, Default, Clone)]
pub enum Slot {
    #[default]
    None,
    Input,
    Output,
    Bidirectional,
}

#[derive(Component, Debug, Clone)]
pub struct Pipe {
    pub source: Option<FluidId>,
    pub sink: Option<FluidId>,
    /// Input/output slots.
    ///
    /// Side indices:
    /// ```text
    ///    0
    /// 3 |P| 1
    ///    2
    /// ```
    pub slots: [Slot; 4],
    /// Progress as a float from 0 to 1
    pub progress: f32,
    /// How fast the progress fills, 1/s
    pub progress_rate: f32,
    pub internal_routing: Vec<InternalRouting>,
    pub model: Handle<Scene>,
    pub locked: bool,
}

#[derive(Debug, Clone)]
pub enum Function {
    Passthrough,
    Mix,
}

/// Pipe routing internal to tile.
///
/// Slot IDs 0 through 3 correspond to I/O slots, 4-99 are internal containers
/// used for internal functions like mixing. Slot 100 is internal source, and 101 is internal sink.
#[derive(Debug, Clone)]
pub struct InternalRouting {
    to: SlotId,
    from: SlotId,
    function: Function,
}

impl InternalRouting {
    pub fn passthrough(from: SlotId, to: SlotId) -> Self {
        InternalRouting {
            to,
            from,
            function: Function::Passthrough,
        }
    }

    pub fn mix(from: SlotId, to: SlotId) -> Self {
        InternalRouting {
            to,
            from,
            function: Function::Mix,
        }
    }
}
