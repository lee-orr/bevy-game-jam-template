use bevy::{
    prelude::*,
    reflect::{Reflect, TypeUuid},
    utils::HashMap,
};
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy_inspector_egui::InspectorOptions;
use serde::Deserialize;

use crate::materialized_scene::{MaterializedScene, MaterializedSceneReference};

pub struct LocationPlugin;

impl Plugin for LocationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(YamlAssetPlugin::<Locations>::new(&["lc.yaml"]));
    }
}

#[derive(Reflect, InspectorOptions, Deserialize, TypeUuid)]
#[uuid = "69d75e22-5894-4b5f-85c3-13b4ee333dd9"]
pub struct Locations(HashMap<String, LocationReference>);

impl Locations {
    pub fn get(&self, key: &str) -> Option<&LocationReference> {
        self.0.get(key)
    }
}

#[derive(Reflect, Deserialize, InspectorOptions, Clone, Debug)]
pub struct LocationReference {
    pub name: String,
    pub scene: MaterializedSceneReference,
    pub challenger_slots: usize,
    pub fog: Option<(Color, f32, f32)>,
    pub ambient: Option<(Color, f32)>,
}

#[derive(Reflect, InspectorOptions)]
pub struct Location {
    pub name: String,
    pub scene: MaterializedScene,
}
