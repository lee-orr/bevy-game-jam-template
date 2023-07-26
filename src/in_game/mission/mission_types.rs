use bevy::{prelude::*, reflect::TypeUuid, utils::HashMap};
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use bevy_turborand::TurboRand;
use serde::Deserialize;

use crate::in_game::encounter::encounter_setup_types::{EncounterInitialDetails, Encounters};

pub struct MissionAssetsPlugin;

impl Plugin for MissionAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Mission>()
            .register_type::<Missions>()
            .register_type::<MissionStage>()
            .add_plugins(YamlAssetPlugin::<Missions>::new(&["ms.yaml"]));
    }
}

#[derive(Resource, Default, Reflect, InspectorOptions, Clone)]
#[reflect(Resource, InspectorOptions)]
pub struct Mission {
    pub title: String,
    pub encounters: Vec<Vec<EncounterInitialDetails>>,
}

#[derive(Resource, Default, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct MissionStage(pub usize);

#[derive(Default, Reflect, InspectorOptions, Deserialize)]
#[reflect(InspectorOptions)]
pub struct MissionGenerationInfo {
    pub titles: Vec<String>,
    pub encounters: Vec<Vec<String>>,
}

impl MissionGenerationInfo {
    pub fn mission(&self, rng: &mut impl TurboRand, encounters: &Encounters) -> Mission {
        let title = rng
            .sample(&self.titles)
            .cloned()
            .unwrap_or("Mission".to_string());
        let encounters = self
            .encounters
            .iter()
            .map(|encounters| rng.sample_multiple(encounters, 2))
            .map(|encounter_names| {
                encounter_names
                    .iter()
                    .filter_map(|encounter_name| encounters.0.get(encounter_name.as_str()).cloned())
                    .collect()
            })
            .collect();
        Mission { title, encounters }
    }
}

#[derive(Resource, Default, Reflect, InspectorOptions, Deserialize, TypeUuid)]
#[reflect(Resource, InspectorOptions)]
#[uuid = "2cc8fe4a-f06d-4aff-b863-ae1a5b743acd"]
pub struct Missions(pub HashMap<String, MissionGenerationInfo>);
