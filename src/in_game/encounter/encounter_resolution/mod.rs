use bevy::prelude::*;

use crate::in_game::game_state::GameState;

use super::{challenger::Challenger, sequencing::EncounterState};

pub struct EncounterResolutionPlugin;

impl Plugin for EncounterResolutionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            check_encounter_state.run_if(in_state(EncounterState::CheckEncounterResolution)),
        )
        .add_systems(OnEnter(EncounterState::EncounterResolved), exit_encounter);
    }
}

#[derive(Component)]
pub struct ChallengerCompleted;

fn check_encounter_state(
    mut commands: Commands,
    query: Query<Entity, (With<Challenger>, Without<ChallengerCompleted>)>,
) {
    info!("Checking encounter state...");
    for e in query.iter() {
        info!("Found {e:?} is still active");
    }
    let next_state = if !query.is_empty() {
        info!("Choosing Action");
        EncounterState::ActionChoice
    } else {
        info!("Encounter Resolved");
        EncounterState::EncounterResolved
    };

    commands.insert_resource(NextState(Some(next_state)));
}

fn exit_encounter(mut commands: Commands) {
    commands.insert_resource(NextState(Some(GameState::Mission)));
}
