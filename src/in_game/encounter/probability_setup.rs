use bevy::{ecs::query::Has, prelude::*};
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use bevy_turborand::{DelegatedRng, GlobalRng};

use super::{
    actions::{ActionChoice, ChallengerAction, Resolution},
    dice_pools::*,
    powers::{Power, PowerTargetingType},
    sequencing::EncounterState,
};

use bevy_ui_dsl::*;

use crate::{
    assets::MainGameAssets,
    in_game::InGameUpdate,
    ui::{
        buttons::{focus_button, focus_text_button, focused_button_activated},
        classes::*,
        intermediary_node_bundles::*,
        DisplayBundle,
    },
};

mod dice_pools;

mod handle_powers;

mod iter_averager;

pub struct ProbabilitySetupPlugin;

impl Plugin for ProbabilitySetupPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DiceType>()
            .register_type::<DicePoolType>()
            .register_type::<DicePool>()
            .register_type::<InitialPools>()
            .register_type::<PowerTargetingType>()
            .register_type::<Power>()
            .register_type::<TargetingTypes>()
            .register_type::<Buttons>()
            .add_systems(
                OnEnter(EncounterState::ProbabilitySetup),
                (setup, dice_pools::setup_initial_pools),
            )
            .add_systems(
                OnExit(EncounterState::ProbabilitySetup),
                (resolve_actions, exit),
            )
            .add_systems(
                InGameUpdate,
                (
                    dice_pools::update_dice_pool_display
                        .before(dice_pools::clear_updated_dice_pool),
                    handle_powers::update_powers.before(handle_powers::clear_updated_powers),
                    dice_pools::clear_updated_dice_pool,
                    handle_powers::clear_updated_powers,
                    dice_pools::update_probability_distibution,
                    handle_powers::update_current_focusables,
                    handle_powers::update_resolve_button,
                    focused_button_activated.pipe(handle_powers::process_input),
                )
                    .run_if(in_state(EncounterState::ProbabilitySetup)),
            );
    }
}

#[derive(Component)]
struct Screen;

#[derive(Component)]
struct DicePoolControl(Entity);

#[derive(Component)]
struct ProbabilityVisualizer(Entity, Vec<(u8, f32)>);

#[derive(Component)]
pub struct PowerContainer;

#[derive(Component, InspectorOptions, Reflect, Default, PartialEq, Eq)]
pub enum Buttons {
    #[default]
    Resolve,
    Power(Entity),
    Pool {
        pool: Entity,
        action: Entity,
    },
    Action(Entity),
}

#[derive(Resource, Reflect, InspectorOptions, Default)]
#[reflect(Resource, InspectorOptions)]
pub enum TargetingTypes {
    #[default]
    SelectPower,
    PowerTarget(PowerTargetingType, Entity, Power),
}

#[derive(Component)]
struct UpdatedDicePool;

#[derive(Component)]
pub struct UpdatePowers;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    actions: Query<(Entity, &ActionChoice, Has<ChallengerAction>)>,
) {
    commands.insert_resource(TargetingTypes::SelectPower);
    let mut dice_pool_controls = Vec::new();
    let mut probability_visualizers = Vec::new();
    let mut action_buttons = Vec::new();
    let mut resolve_button = None;
    let mut power_container = None;
    let r = root(
        c_probability_setup_root,
        &asset_server,
        &mut commands,
        |p| {
            node(probability_grid, p, |p| {
                for (entity, choice, is_challenger) in actions.iter() {
                    action_buttons.push((
                        focus_button(
                            (
                                probability_card.nb(),
                                if is_challenger {
                                    challenger_card.nb()
                                } else {
                                    player_card.nb()
                                },
                            ),
                            if is_challenger {
                                apply_action_state_ch
                            } else {
                                apply_action_state_pl
                            },
                            p,
                            |p| {
                                node(probability_card_title.nb(), p, |p| {
                                    text(
                                        choice.title.as_str(),
                                        (),
                                        (probability_card_title_text, druid_text),
                                        p,
                                    );
                                });

                                dice_pool_controls.push((
                                    node(probability_card_dice_pool_container.nb(), p, |_| {}),
                                    entity,
                                ));
                                probability_visualizers.push((
                                    node(probability_card_visualizer.nb(), p, |_| {}),
                                    entity,
                                ));
                            },
                        ),
                        entity,
                    ));
                }
            });
            node((probability_power_container, probability_grid), p, |p| {
                node((), p, |_| {}).set(&mut power_container);
                focus_text_button(
                    "Resolve!",
                    (c_button.nb(), primary_box_item.nb()),
                    apply_button_state,
                    button_text,
                    p,
                )
                .set(&mut resolve_button);
            });
        },
    );
    commands.entity(r).insert(Screen);

    if let Some(resolve_button) = resolve_button {
        commands.entity(resolve_button).insert(Buttons::Resolve);
    }
    if let Some(power_container) = power_container {
        commands
            .entity(power_container)
            .insert((PowerContainer, UpdatePowers));
    }

    for (ctl, target) in dice_pool_controls.iter() {
        commands.entity(*ctl).insert(DicePoolControl(*target));
    }
    for (ctl, target) in probability_visualizers.iter() {
        commands
            .entity(*ctl)
            .insert(ProbabilityVisualizer(*target, vec![]));
    }
    for (ctl, target) in action_buttons.iter() {
        commands.entity(*ctl).insert(Buttons::Action(*target));
    }
}

fn exit(mut commands: Commands, query: Query<Entity, With<Screen>>) {
    info!("Exiting Probability Resolution");
    for item in query.iter() {
        commands.entity(item).despawn_recursive();
    }
}

fn resolve_actions(
    mut commands: Commands,
    dice_pools: Query<&DicePool>,
    updated_actions: Query<(Entity, &ActionChoice, &Children)>,
    mut global_rng: ResMut<GlobalRng>,
) {
    for (entity, action, dice_pool_entities) in updated_actions.iter() {
        let dice_pools = dice_pool_entities
            .iter()
            .flat_map(|e| dice_pools.get(*e).ok())
            .collect::<Vec<_>>();
        let roll = dice_pools.as_slice().roll(global_rng.get_mut());
        let (result, gap) = action.evaluate(roll);
        commands
            .entity(entity)
            .despawn_descendants()
            .insert(Resolution { roll, result, gap });
    }
}
