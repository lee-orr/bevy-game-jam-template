pub mod mission_types;

use bevy::prelude::*;

use bevy_inspector_egui::InspectorOptions;
use bevy_turborand::{DelegatedRng, GlobalRng, TurboRand};
use bevy_ui_dsl::{node, root, text};

use crate::{
    assets::MainGameAssets,
    ui::{
        buttons::{focus_button, focused_button_activated, TypedFocusedButtonQuery},
        classes::*,
        intermediary_node_bundles::IntoIntermediaryNodeBundle,
        DisplayBundle,
    },
};

use self::mission_types::{Mission, MissionAssetsPlugin, MissionStage};

use super::{
    encounter::{
        dice_pools::DiceType,
        encounter_setup_types::{self},
        powers::Power,
    },
    game_state::GameState,
    story::PhaseRound,
    InGameUpdate,
};

pub struct MissionPlugin;

impl Plugin for MissionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MissionAssetsPlugin)
            .register_type::<UiButton>()
            .add_systems(
                OnEnter(GameState::Mission),
                (draw_encounter_selection_ui, draw_completed_ui),
            )
            .add_systems(OnExit(GameState::Mission), clear_world_map)
            .add_systems(
                InGameUpdate,
                (focused_button_activated.pipe(process_input)).run_if(in_state(GameState::Mission)),
            );
    }
}

#[derive(Component)]
pub struct MissionEntity;

#[derive(Component, Reflect, InspectorOptions)]
pub struct UiButton(Option<encounter_setup_types::EncounterInitialDetails>);

fn clear_world_map(mut commands: Commands, mission_entities: Query<Entity, With<MissionEntity>>) {
    for entity in mission_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn draw_encounter_selection_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mission: Res<Mission>,
    mission_stage: Res<MissionStage>,
) {
    let Some(encounters) = mission.encounters.get(mission_stage.0) else {
        return;
    };

    let title = mission.title.as_str();

    let mut buttons = Vec::new();

    let r = root(mission_root, &asset_server, &mut commands, |p| {
        node(mission_container, p, |p| {
            node(mission_encounter_title.nb(), p, |p| {
                text(title, (), main_text, p);
            });

            buttons = encounters
                .iter()
                .map(|encounter| {
                    let button = focus_button(
                        match encounter.player_faction {
                            super::factions::Faction::Knights => encounter_knight_listing,
                            super::factions::Faction::Druids => encounter_druid_listing,
                        }
                        .nb(),
                        match encounter.player_faction {
                            super::factions::Faction::Knights => apply_encounter_knight_state,
                            super::factions::Faction::Druids => apply_encounter_druid_state,
                        },
                        p,
                        |p| {
                            text(
                                encounter.title.clone().unwrap_or("Encounter".to_string()),
                                (),
                                (
                                    standard_text,
                                    button_text,
                                    match encounter.player_faction {
                                        super::factions::Faction::Knights => knight_text,
                                        super::factions::Faction::Druids => druid_text,
                                    },
                                ),
                                p,
                            );
                        },
                    );
                    (button, encounter)
                })
                .collect();
        });
    });
    commands.entity(r).insert(MissionEntity);
    for (button, encounter) in buttons.into_iter() {
        commands
            .entity(button)
            .insert(UiButton(Some(encounter.clone())));
    }
}

fn process_input(
    In(focused): In<Option<Entity>>,
    mut commands: Commands,
    interaction_query: TypedFocusedButtonQuery<'_, '_, '_, UiButton>,
    mission_stage: Option<Res<MissionStage>>,
    mut phase_round: ResMut<PhaseRound>,
) {
    let Some(mission_stage) = mission_stage else {
        return;
    };
    let Some(focused) = focused else {
        return;
    };
    let Some((_, btn)) = interaction_query.get(focused).ok() else {
        return;
    };
    if let Some(mission) = &btn.0 {
        commands.insert_resource(mission.clone());
        commands.insert_resource(MissionStage(mission_stage.0 + 1));
        commands.insert_resource(NextState(Some(GameState::Encounter)));
    } else {
        commands.remove_resource::<Mission>();
        commands.remove_resource::<MissionStage>();
        phase_round.0 += 1;
        commands.insert_resource(NextState(Some(GameState::WorldMap)));
    }
}

fn draw_completed_ui(
    mission_stage: Res<MissionStage>,
    mission: Res<Mission>,
    assets: Res<MainGameAssets>,
    asset_server: Res<AssetServer>,
    _phase_round: ResMut<PhaseRound>,
    mut commands: Commands,
    mut global_rng: ResMut<GlobalRng>,
) {
    if mission_stage.0 < mission.encounters.len() {
        return;
    }

    let title = mission.title.as_str();

    let mut buttons = None;

    let rng = global_rng.get_mut();
    let new_powers = rng.sample_multiple(
        &[
            Power::AddDice(DiceType::D4),
            Power::AddDice(DiceType::D4),
            Power::AddDice(DiceType::D4),
            Power::AddDice(DiceType::D4),
            Power::AddDice(DiceType::D6),
            Power::AddDice(DiceType::D6),
            Power::AddDice(DiceType::D8),
            Power::AddDice(DiceType::D12),
            Power::Advantage,
            Power::Advantage,
            Power::Advantage,
            Power::Advantage,
            Power::Advantage,
            Power::SplitDice,
            Power::SplitDice,
            Power::SplitDice,
            Power::SplitDice,
            Power::StaticBonus(1),
            Power::StaticBonus(1),
            Power::StaticBonus(1),
            Power::StaticBonus(1),
            Power::StaticBonus(2),
            Power::StaticBonus(2),
            Power::StaticBonus(3),
        ],
        4,
    );

    for item in new_powers.iter() {
        commands.spawn(**item);
    }

    let r = root(mission_root, &asset_server, &mut commands, |p| {
        node(mission_container, p, |p| {
            node(mission_encounter_title.nb(), p, |p| {
                text(format!("{title} Complete"), (), (main_text, knight_text), p);
            });

            text("Gained the following rewards:", (), standard_text, p);

            node((), p, |p| {
                for item in new_powers.iter() {
                    item.display_bundle(&assets, 50., p);
                }
            });

            buttons = Some(focus_button(
                encounter_listing.nb(),
                apply_encounter_state,
                p,
                |p| {
                    text("Return to World Map", (), standard_text, p);
                },
            ));
        });
    });
    commands.entity(r).insert(MissionEntity);
    if let Some(button) = buttons {
        commands.entity(button).insert(UiButton(None));
    }
}
