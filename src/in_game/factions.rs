use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use serde::Deserialize;
#[derive(Debug, Clone, Copy, Reflect, Deserialize, InspectorOptions)]
pub enum Faction {
    Knights,
    Druids,
}
