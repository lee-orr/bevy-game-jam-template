use bevy::{
    prelude::*,
    reflect::{Reflect, TypeUuid},
    utils::HashMap,
};
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy_inspector_egui::InspectorOptions;
use bevy_turborand::{DelegatedRng, GlobalRng, TurboRand};
use serde::Deserialize;

use crate::materialized_scene::MaterializedSceneReference;

use super::{
    actions::{
        ActionChoice, ActionDefinition, ActionTarget, ChallengerActionBundle, PlayerActionBundle,
    },
    encounter_resolution::ChallengerCompleted,
    health::MaxHealth,
    player::Player,
    sequencing::{EncounterState, PublishAvailableActions},
};

pub struct ChallengerPlugin;

impl Plugin for ChallengerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<ChallengerReference>()
            .register_type::<Challenger>()
            .register_type::<Challengers>()
            .add_plugins(YamlAssetPlugin::<Challengers>::new(&["ch.yaml"]))
            .add_systems(
                OnEnter(EncounterState::ActionChoice),
                publish_challenger_action.in_set(PublishAvailableActions),
            );
    }
}

#[derive(Reflect, InspectorOptions, Deserialize, Clone, Debug)]
pub struct ChallengerReference {
    pub name: String,
    pub scene: MaterializedSceneReference,
    pub available_actions: Vec<ActionDefinition>,
    pub published_actions: Vec<ActionDefinition>,
    pub health: Option<MaxHealth>,
}

#[derive(Reflect, InspectorOptions, Component)]
pub struct Challenger {
    pub id: usize,
    pub name: String,
    pub available_actions: Vec<ActionDefinition>,
    pub published_actions: Vec<ActionDefinition>,
}

#[derive(Reflect, InspectorOptions, Deserialize, TypeUuid)]
#[uuid = "e3cb22e9-0e2b-4af0-be00-c9c3fc18dbc7"]
pub struct Challengers(HashMap<String, ChallengerReference>);

impl Challengers {
    pub fn get(&self, key: &str) -> Option<&ChallengerReference> {
        self.0.get(key)
    }
}

fn publish_challenger_action(
    mut commands: Commands,
    challengers: Query<(Entity, &Challenger), Without<ChallengerCompleted>>,
    players: Query<Entity, With<Player>>,
    mut global_rng: ResMut<GlobalRng>,
) {
    let rng = global_rng.get_mut();

    for (entity, challenger) in challengers.iter() {
        let Some(choice) = rng.sample(&challenger.available_actions) else {
            continue;
        };

        for player in players.iter() {
            commands.entity(entity).with_children(|p| {
                p.spawn(ChallengerActionBundle {
                    action_choice: ActionChoice {
                        title: choice.choice.title.replace("**", &challenger.name),
                        content: choice.choice.content.replace("**", &challenger.name),
                        ..choice.choice.clone()
                    },
                    action_type: choice.action_type.clone(),
                    target: ActionTarget(Some(player)),
                    ..default()
                });
            });
            commands.entity(player).with_children(|p| {
                for choice in challenger.published_actions.iter() {
                    p.spawn(PlayerActionBundle {
                        action_choice: ActionChoice {
                            title: choice.choice.title.replace("**", &challenger.name),
                            content: choice.choice.content.replace("**", &challenger.name),
                            ..choice.choice.clone()
                        },
                        action_type: choice.action_type.clone(),
                        target: ActionTarget(Some(entity)),
                    });
                }
            });
        }
    }
}
