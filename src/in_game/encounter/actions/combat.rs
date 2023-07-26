use bevy::{ecs::query::Has, prelude::*};
use bevy_ui_dsl::*;

use crate::{
    in_game::{
        encounter::{
            action_resolutions::ActiveResolution, challenger::Challenger,
            encounter_resolution::ChallengerCompleted, health::CurrentHealth, player::Player,
            sequencing::EncounterState,
        },
        game_state::GameState,
        InGameUpdate,
    },
    ui::{
        buttons::{focus_text_button, focused_button_activated},
        classes::*,
        intermediary_node_bundles::IntoIntermediaryNodeBundle,
    },
};

use super::{ActionChoice, ActionTarget, ActionType, ChallengerAction, Resolution};

pub struct CombatActionPlugin;

impl Plugin for CombatActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (display_combat_resolution).run_if(in_state(EncounterState::OutcomeResolution)),
        )
        .add_systems(
            OnExit(EncounterState::OutcomeResolution),
            end_combat_encounter,
        )
        .add_systems(
            InGameUpdate,
            (focused_button_activated.pipe(process_input))
                .run_if(in_state(EncounterState::OutcomeResolution)),
        );
    }
}
#[derive(Component)]
struct Screen;

#[derive(Component)]
pub struct Button;

type CurrentResolutionQuery<'w, 's, 'a> = Query<
    'w,
    's,
    (
        Entity,
        &'a ActionChoice,
        &'a Resolution,
        &'a ActionType,
        &'a ActionTarget,
        Has<ChallengerAction>,
    ),
    Added<ActiveResolution>,
>;

fn display_combat_resolution(
    mut commands: Commands,
    resolution: CurrentResolutionQuery,
    mut targetable: Query<&mut CurrentHealth>,
    asset_server: Res<AssetServer>,
) {
    let Ok((_entity, choice, resolution, action_type, target, is_challanger)) =
        resolution.get_single()
    else {
        return;
    };
    let ActionType::Attack { base_damage } = action_type else {
        return;
    };
    let (result_text, damage) = if is_challanger {
        match resolution.result {
            super::ActionResult::CriticalFail => ("Failed Badly", (*base_damage) * 2),
            super::ActionResult::Fail => ("Failed", *base_damage),
            super::ActionResult::Success => ("Succeeded!", 0),
            super::ActionResult::CriticalSuccess => ("Amazing Success!", 0),
        }
    } else {
        match resolution.result {
            super::ActionResult::CriticalFail => ("Failed Badly", 0),
            super::ActionResult::Fail => ("Failed", 0),
            super::ActionResult::Success => ("Succeeded!", *base_damage),
            super::ActionResult::CriticalSuccess => ("Amazing Success!", (*base_damage) * 2),
        }
    };
    if let ActionTarget(Some(target)) = target {
        if let Ok(mut target) = targetable.get_mut(*target) {
            info!("Target took damage!");
            target.0 = target.0.saturating_sub(damage as usize);
        }
    }
    let mut next_button = None;
    let root = root(c_root, &asset_server, &mut commands, |p| {
        node(primary_box, p, |p| {
            node((span.nb(), primary_box_main.nb()), p, |p| {
                text(&choice.title, (), (main_text, knight_text), p);
            });
            text(result_text, primary_box_item.nb(), standard_text, p);

            if damage > 0 {
                text(
                    if is_challanger {
                        format!("Player took {damage}")
                    } else {
                        format!("Target took {damage}")
                    },
                    primary_box_item.nb(),
                    standard_text,
                    p,
                );
            }

            text(
                format!("Rolled a {}", resolution.roll),
                primary_box_item.nb(),
                standard_text,
                p,
            );
            focus_text_button(
                "Next",
                (c_button.nb(), primary_box_item.nb()),
                apply_button_state,
                button_text,
                p,
            )
            .set(&mut next_button);
        });
    });

    commands.entity(root).insert(Screen);
    if let Some(next_button) = next_button {
        commands.entity(next_button).insert(Button);
    }
}

type CombatantWithUpdatedHealth<'w, 's, 'a> =
    Query<'w, 's, (Entity, &'a CurrentHealth), (With<Challenger>, Changed<CurrentHealth>)>;

fn end_combat_encounter(
    challengers: CombatantWithUpdatedHealth,
    player: Query<&CurrentHealth, With<Player>>,
    mut commands: Commands,
) {
    for player in player.iter() {
        if player.0 == 0 {
            commands.insert_resource(NextState(Some(GameState::Failed)));
        }
    }
    for (entity, challenger) in challengers.iter() {
        if challenger.0 == 0 {
            info!("Challenger Reduced to ZERO {entity:?}");
            commands.entity(entity).insert(ChallengerCompleted);
        }
    }
}

fn process_input(
    In(focused): In<Option<Entity>>,
    mut commands: Commands,
    screen: Query<Entity, With<Screen>>,
    resolved_action: Query<Entity, With<ActiveResolution>>,
) {
    let Some(_) = focused else {
        return;
    };
    for item in screen.iter() {
        commands.entity(item).despawn_recursive();
    }

    for item in resolved_action.iter() {
        commands.entity(item).despawn_recursive();
    }
}
