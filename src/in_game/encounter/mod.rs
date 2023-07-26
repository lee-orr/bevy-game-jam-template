mod action_choice;
mod action_resolutions;
pub mod actions;
mod challenger;
pub mod dice_pools;
mod encounter_assets;
mod encounter_resolution;
mod health;
mod introduction;
pub mod location;
mod player;
pub mod powers;
mod probability_setup;
pub mod sequencing;

pub mod encounter_generation;
pub mod encounter_setup_types;

use bevy::{
    gltf::{Gltf, GltfNode},
    input::common_conditions::input_toggle_active,
    prelude::*,
};
use bevy_asset_loader::prelude::DynamicAssets;
use bevy_inspector_egui::quick::StateInspectorPlugin;

use crate::{
    in_game::encounter::{challenger::Challenger, health::CurrentHealth, player::Player},
    materialized_scene::MaterializedSceneBundle,
    ui::colors::{DEFAULT_AMBIENT, DEFAULT_CLEAR},
};

use self::{
    action_choice::ActionChoicePlugin,
    action_resolutions::ActionResolutionPlugin,
    actions::ActionPlugin,
    challenger::ChallengerPlugin,
    encounter_assets::{
        setup_encounter_assets, EncounterAssetPlugin, EncounterAssets, Materials, SceneBundler,
    },
    encounter_generation::generate_encounter,
    encounter_resolution::EncounterResolutionPlugin,
    encounter_setup_types::EncounterSetupPlugin,
    health::HealthPlugin,
    introduction::IntroductionPlugin,
    location::LocationPlugin,
    player::PlayerPlugin,
    probability_setup::ProbabilitySetupPlugin,
    sequencing::EncounterState,
};

use super::{factions::Faction, game_state::GameState, InGameUpdate};

pub use self::challenger::Challengers;
pub use self::location::Locations;
pub use self::player::Players;

pub struct EncounterPlugin;

impl Plugin for EncounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<EncounterState>()
            .register_type::<EncounterState>()
            .add_plugins(
                StateInspectorPlugin::<EncounterState>::default()
                    .run_if(input_toggle_active(false, KeyCode::F1)),
            )
            .add_plugins((
                IntroductionPlugin,
                EncounterAssetPlugin,
                LocationPlugin,
                ChallengerPlugin,
                PlayerPlugin,
                ActionChoicePlugin,
                ProbabilitySetupPlugin,
                HealthPlugin,
                ActionResolutionPlugin,
                ActionPlugin,
                EncounterResolutionPlugin,
                EncounterSetupPlugin,
            ))
            .add_systems(
                OnEnter(GameState::Encounter),
                generate_encounter.run_if(not(resource_exists::<
                    encounter_setup_types::EncounterSetup,
                >())),
            )
            .add_systems(OnExit(GameState::Encounter), despawn_encounter)
            .add_systems(OnEnter(EncounterState::Introduction), spawn_encounter)
            .add_systems(
                InGameUpdate,
                start_encounter.run_if(
                    in_state(EncounterState::None)
                        .and_then(resource_exists::<encounter_setup_types::EncounterSetup>())
                        .and_then(resource_changed::<encounter_setup_types::EncounterSetup>()),
                ),
            );
    }
}

#[derive(Component)]
pub struct EncounterEntity;

fn start_encounter(
    mut commands: Commands,
    setup: Res<encounter_setup_types::EncounterSetup>,
    mut dynamic_assets: ResMut<DynamicAssets>,
) {
    setup_encounter_assets(setup.as_ref(), dynamic_assets.as_mut());
    commands.insert_resource(NextState(Some(EncounterState::Loading)));
}

fn spawn_encounter(
    mut commands: Commands,
    setup: Res<encounter_setup_types::EncounterSetup>,
    assets: Res<EncounterAssets>,
    materials: Res<Materials>,
    gltf: Res<Assets<Gltf>>,
    gltf_node: Res<Assets<GltfNode>>,
    camera: Query<Entity, With<Camera3d>>,
) {
    let bundler = SceneBundler::new(&assets, &materials, &gltf, &gltf_node);
    if let (Some(location), Some(player)) = (&setup.location, &setup.player) {
        info!("Spawning Location {location:?}");
        if let Some(bundle) = bundler.scene(&location.scene) {
            commands.spawn((bundle, EncounterEntity));
        } else {
            error!("Couldn't setup bundle");
        }

        if let Some(fog) = location.fog {
            commands.insert_resource(ClearColor(fog.0));
            for camera in camera.iter() {
                commands.entity(camera).insert(FogSettings {
                    color: fog.0,
                    falloff: FogFalloff::Linear {
                        start: fog.1,
                        end: fog.2,
                    },
                    ..Default::default()
                });
            }
        } else {
            commands.insert_resource(ClearColor(DEFAULT_CLEAR));

            for camera in camera.iter() {
                commands.entity(camera).remove::<FogSettings>();
            }
        }

        if let Some((color, brightness)) = location.ambient {
            commands.insert_resource(AmbientLight { color, brightness });
        } else {
            commands.insert_resource(DEFAULT_AMBIENT);
        }

        if let Some(transform) = bundler.camera_position(&location.scene) {
            info!("Placing Camera");
            for camera in camera.iter() {
                commands.entity(camera).insert(transform);
            }
        }

        if let (Some(transform), Some(bundle)) = (
            bundler.player_position(&location.scene),
            bundler.scene(&player.scene),
        ) {
            info!("Placing Player");
            let bundle = MaterializedSceneBundle {
                transform: TransformBundle {
                    local: transform,
                    global: GlobalTransform::default(),
                },
                ..bundle.clone()
            };
            commands.spawn((
                Player {
                    name: player.name.clone(),
                    combat_actions: player.combat_actions.clone(),
                },
                bundle,
                EncounterEntity,
                CurrentHealth(player.health.0),
                player.health,
                Name::new("Player"),
            ));
        }

        let mut challenger_id = 0usize;
        let challenger_slots = location.challenger_slots;

        for (count, challenger) in setup.challengers.iter() {
            if challenger_id >= challenger_slots {
                break;
            }
            if let Some(bundle) = bundler.scene(&challenger.scene) {
                for _ in 0..*count {
                    if challenger_id >= challenger_slots {
                        break;
                    }

                    info!("Placing Challenger {challenger_id}");
                    let Some(transform) =
                        bundler.challenger_position(&location.scene, challenger_id)
                    else {
                        break;
                    };

                    let bundle = MaterializedSceneBundle {
                        transform: TransformBundle {
                            local: transform,
                            global: GlobalTransform::default(),
                        },
                        ..bundle.clone()
                    };
                    let mut entity = commands.spawn((
                        Challenger {
                            id: challenger_id,
                            name: challenger.name.clone(),
                            available_actions: challenger.available_actions.clone(),
                            published_actions: challenger.published_actions.clone(),
                        },
                        bundle,
                        EncounterEntity,
                        Name::new(format!("{} - {challenger_id}", challenger.name)),
                    ));
                    if let Some(health) = challenger.health {
                        entity.insert((health, CurrentHealth(health.0)));
                    }
                    challenger_id += 1;
                }
            }
        }
    }
}

fn despawn_encounter(mut commands: Commands, query: Query<Entity, With<EncounterEntity>>) {
    commands.remove_resource::<encounter_setup_types::EncounterSetup>();
    commands.insert_resource(NextState(Some(EncounterState::None)));
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
