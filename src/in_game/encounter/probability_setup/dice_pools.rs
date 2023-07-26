use crate::{
    assets::MainGameAssets,
    in_game::encounter::{
        actions::{ActionChoice, ActionResult},
        dice_pools::DicePool,
    },
    ui::{buttons::*, colors, DisplayBundle},
};
use bevy::prelude::*;
use bevy_ui_dsl::root;

use super::*;

pub(crate) fn setup_initial_pools(mut commands: Commands, query: Query<(Entity, &ActionChoice)>) {
    for (entity, choice) in query.iter() {
        commands
            .entity(entity)
            .insert(UpdatedDicePool)
            .with_children(|p| {
                for pool in choice.dice_pool.iter() {
                    p.spawn(*pool);
                }
            });
    }
}

pub(super) fn update_dice_pool_display(
    mut commands: Commands,
    dice_pools: Query<&DicePool>,
    updated_actions: Query<&Children, With<UpdatedDicePool>>,
    dice_pool_display: Query<(Entity, &DicePoolControl)>,
    asset_server: Res<AssetServer>,
    assets: Res<MainGameAssets>,
) {
    for (display_entity, DicePoolControl(action_entity)) in dice_pool_display.iter() {
        let Ok(dice_pool_entities) = updated_actions.get(*action_entity) else {
            continue;
        };
        let mut dice_pool_buttons = Vec::new();

        let dice_pool_root = root((), &asset_server, &mut commands, |p| {
            for child in dice_pool_entities.iter() {
                let Ok(dice_pool) = dice_pools.get(*child) else {
                    continue;
                };
                dice_pool_buttons.push((
                    focus_button(power_card_container.nb(), apply_power_card_state, p, |p| {
                        dice_pool.display_bundle(&assets, 40., p)
                    }),
                    *child,
                ));
            }
        });
        commands
            .entity(display_entity)
            .despawn_descendants()
            .add_child(dice_pool_root);

        for (button, pool) in dice_pool_buttons.iter() {
            commands.entity(*button).insert(Buttons::Pool {
                pool: *pool,
                action: *action_entity,
            });
        }
    }
}

pub(super) fn clear_updated_dice_pool(
    mut commands: Commands,
    actions: Query<Entity, With<UpdatedDicePool>>,
) {
    for action in actions.iter() {
        commands.entity(action).remove::<UpdatedDicePool>();
    }
}

pub(super) fn update_probability_distibution(
    mut commands: Commands,
    dice_pools: Query<&DicePool>,
    updated_actions: Query<(&Children, &ActionChoice, Has<UpdatedDicePool>)>,
    dice_pool_display: Query<(Entity, &ProbabilityVisualizer)>,
    mut global_rng: ResMut<GlobalRng>,
) {
    for (display_entity, ProbabilityVisualizer(action_entity, stored_simulation)) in
        dice_pool_display.iter()
    {
        let Ok((dice_pool_entities, action, just_updated)) = updated_actions.get(*action_entity)
        else {
            continue;
        };
        let dice_pools = dice_pool_entities
            .iter()
            .flat_map(|e| dice_pools.get(*e).ok())
            .collect::<Vec<_>>();

        let simulation =
            SimulateDice::<100>::simulate(&dice_pools.as_slice(), global_rng.get_mut());

        let simulation = if just_updated {
            simulation
        } else {
            iter_averager::Averager::<'_, _, _, 1, 20>(
                simulation.iter().peekable(),
                stored_simulation.iter().peekable(),
            )
            .collect::<Vec<(u8, f32)>>()
        };

        commands
            .entity(display_entity)
            .despawn_descendants()
            .with_children(|p| {
                for (value, rate) in simulation.iter() {
                    let (result_type, _) = action.evaluate(*value);
                    let result_type = match result_type {
                        ActionResult::CriticalFail => colors::CRITICAL_FAIL_COLOR,
                        ActionResult::Fail => colors::FAIL_COLOR,
                        ActionResult::Success => colors::SUCCESS_COLOR,
                        ActionResult::CriticalSuccess => colors::CRITICAL_COLOR,
                    };
                    p.spawn(NodeBundle {
                        style: Style {
                            height: Val::Percent(*rate * 100.),
                            flex_grow: 1.,
                            flex_shrink: 1.,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|p| {
                        p.spawn(NodeBundle {
                            style: Style {
                                height: Val::Percent(100.),
                                position_type: PositionType::Absolute,
                                top: Val::Px(0.),
                                left: Val::Px(1.5),
                                right: Val::Px(1.5),
                                ..Default::default()
                            },
                            background_color: BackgroundColor(result_type),
                            ..Default::default()
                        });
                    });
                }
            })
            .insert(ProbabilityVisualizer(*action_entity, simulation));
    }
}
