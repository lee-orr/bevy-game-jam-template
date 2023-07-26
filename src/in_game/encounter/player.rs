use bevy::{
    prelude::*,
    reflect::{Reflect, TypeUuid},
    utils::HashMap,
};
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy_inspector_egui::InspectorOptions;
use serde::Deserialize;

use crate::materialized_scene::MaterializedSceneReference;

use super::{
    actions::{ActionChoice, ActionDefinition, ActionTarget, ActionType, PlayerActionBundle},
    challenger::Challenger,
    encounter_resolution::ChallengerCompleted,
    health::{CurrentHealth, MaxHealth},
    sequencing::{EncounterState, PublishAvailableActions},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<PlayerReference>()
            .register_type::<Player>()
            .register_type::<Players>()
            .add_plugins(YamlAssetPlugin::<Players>::new(&["pl.yaml"]))
            .add_systems(
                OnEnter(EncounterState::ActionChoice),
                publish_combat_actions.in_set(PublishAvailableActions),
            );
    }
}

#[derive(Reflect, InspectorOptions, Deserialize, Clone, Debug)]
pub struct PlayerReference {
    pub name: String,
    pub scene: MaterializedSceneReference,
    pub combat_actions: Vec<ActionDefinition>,
    pub health: MaxHealth,
}

#[derive(Reflect, InspectorOptions, Component)]
pub struct Player {
    pub name: String,
    pub combat_actions: Vec<ActionDefinition>,
}

#[derive(Reflect, InspectorOptions, Deserialize, TypeUuid)]
#[uuid = "4c70a2d8-8e22-4a7a-9bee-289fb6d417e8"]
pub struct Players(HashMap<String, PlayerReference>);

impl Players {
    pub fn get(&self, key: &str) -> Option<&PlayerReference> {
        self.0.get(key)
    }
}

fn publish_combat_actions(
    mut commands: Commands,
    players: Query<(Entity, &Player)>,
    challengers: Query<(Entity, &Challenger, &CurrentHealth), Without<ChallengerCompleted>>,
) {
    for (entity, player) in players.iter() {
        commands.entity(entity).with_children(|p| {
            for (ch_entity, challenger, _health) in challengers.iter() {
                for action in player.combat_actions.iter() {
                    let action_choice = ActionChoice {
                        title: action.choice.title.replace("**", &challenger.name),
                        ..action.choice.clone()
                    };
                    p.spawn(PlayerActionBundle {
                        action_choice,
                        action_type: match action.action_type {
                            super::actions::ActionType::Attack { base_damage } => {
                                ActionType::Attack { base_damage }
                            }
                            _ => action.action_type.clone(),
                        },
                        target: ActionTarget(Some(ch_entity)),
                    });
                }
            }
        });
    }
}
