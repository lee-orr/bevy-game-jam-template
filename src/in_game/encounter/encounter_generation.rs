use bevy::prelude::*;

use crate::assets::MainGameAssets;

use super::*;

pub fn generate_encounter(
    mut commands: Commands,
    assets: Res<MainGameAssets>,
    locations: Res<Assets<Locations>>,
    challengers: Res<Assets<Challengers>>,
    players: Res<Assets<Players>>,
    initial_details: Option<Res<encounter_setup_types::EncounterInitialDetails>>,
) {
    let (Some(locations), Some(challengers), Some(players)) = (
        locations.get(&assets.locations),
        challengers.get(&assets.challengers),
        players.get(&assets.players),
    ) else {
        return;
    };
    let initial_details = initial_details.map(|v| v.clone()).unwrap_or_default();
    commands.remove_resource::<encounter_setup_types::EncounterInitialDetails>();
    commands.insert_resource(encounter_setup_types::EncounterSetup {
        title: initial_details.title,
        introduction: initial_details.introduction,
        location: initial_details
            .location
            .and_then(|v| locations.get(&v).cloned()),
        player: players
            .get(match initial_details.player_faction {
                Faction::Knights => "player_knight",
                Faction::Druids => "player_druid",
            })
            .cloned(),
        challengers: initial_details
            .challengers
            .iter()
            .filter_map(|(n, v)| challengers.get(v).map(|v| (*n, v.clone())))
            .collect(),
        ..Default::default()
    });
    info!("Generating Encounter");
}
