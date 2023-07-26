use bevy::prelude::*;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

#[derive(Clone, Eq, PartialEq, Copy, Debug, Hash, Default, States, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub enum GameState {
    #[default]
    None,
    WorldMap,
    Mission,
    Encounter,
    Failed,
    Complete,
}

#[derive(Clone, Eq, PartialEq, Copy, Debug, Hash, Default, States, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub enum PauseState {
    #[default]
    None,
    Paused,
}
