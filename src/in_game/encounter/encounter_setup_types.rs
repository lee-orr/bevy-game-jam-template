use bevy::{prelude::*, reflect::TypeUuid, utils::HashMap};
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use serde::Deserialize;

use crate::in_game::factions::Faction;

use super::{
    challenger::ChallengerReference, location::LocationReference, player::PlayerReference,
};

pub struct EncounterSetupPlugin;

impl Plugin for EncounterSetupPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<EncounterSetup>()
            .register_type::<EncounterInitialDetails>()
            .add_plugins(YamlAssetPlugin::<Encounters>::new(&["en.yaml"]));
    }
}

#[derive(Resource, Reflect, InspectorOptions, Clone, Deserialize)]
#[reflect(Resource, InspectorOptions)]
pub struct EncounterInitialDetails {
    pub title: Option<String>,
    pub introduction: Option<String>,
    pub player_faction: Faction,
    pub challengers: Vec<(usize, String)>,
    pub location: Option<String>,
}

impl Default for EncounterInitialDetails {
    fn default() -> Self {
        Self {
            title: Some("An Encounter".to_string()),
            player_faction: Faction::Knights,
            challengers: vec![(1, "monster".to_string())],
            location: Some("grass".to_string()),
            introduction: Some("Let me introduce myself".to_string()),
        }
    }
}

#[derive(Resource, Reflect, InspectorOptions, Clone)]
#[reflect(Resource, InspectorOptions)]
pub struct EncounterSetup {
    pub title: Option<String>,
    pub introduction: Option<String>,
    pub player_faction: Faction,
    pub player: Option<PlayerReference>,
    pub challengers: Vec<(usize, ChallengerReference)>,
    pub location: Option<LocationReference>,
}

impl Default for EncounterSetup {
    fn default() -> Self {
        Self {
            title: Some("An Encounter".to_string()),
            introduction: Some("Let me introduce myself...".to_string()),
            player_faction: Faction::Knights,
            challengers: vec![],
            location: None,
            player: None,
        }
    }
}

#[derive(Reflect, InspectorOptions, Deserialize, TypeUuid)]
#[uuid = "274d4ad0-c00d-4889-b3dd-bc0d688ddc40"]
pub struct Encounters(pub HashMap<String, EncounterInitialDetails>);
