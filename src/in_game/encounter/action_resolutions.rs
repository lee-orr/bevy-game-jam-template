use bevy::prelude::*;

use crate::in_game::InGameUpdate;

use super::{
    actions::{ActionChoice, ChallengerAction, Resolution},
    sequencing::EncounterState,
};

pub struct ActionResolutionPlugin;

impl Plugin for ActionResolutionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            InGameUpdate,
            trigger_next_resolution.run_if(in_state(EncounterState::OutcomeResolution)),
        );
    }
}

#[derive(Component)]
pub struct ActiveResolution;

type ResolvedActionsQuery<'w, 's, 'a> =
    Query<'w, 's, (Entity, Option<&'a ChallengerAction>), (With<ActionChoice>, With<Resolution>)>;

fn trigger_next_resolution(
    mut commands: Commands,
    resolved_actions: ResolvedActionsQuery,
    active_resolution: Query<Entity, With<ActiveResolution>>,
) {
    if !active_resolution.is_empty() {
        return;
    }
    info!("Looking for next resolution!");
    let next_action = resolved_actions.iter().fold(
        None,
        |resolved: Option<(Entity, Option<&ChallengerAction>)>, (entity, challanger_action)| {
            if let Some(resolved) = resolved {
                match resolved.1 {
                    Some(_) => Some((entity, challanger_action)),
                    None => Some(resolved),
                }
            } else {
                Some((entity, challanger_action))
            }
        },
    );

    match next_action {
        Some((entity, _)) => {
            commands.entity(entity).insert(ActiveResolution);
        }
        None => {
            commands.insert_resource(NextState(Some(EncounterState::CheckEncounterResolution)));
        }
    }
}
