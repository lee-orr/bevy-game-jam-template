use crate::ui::buttons::TypedFocusedButtonQuery;
use bevy::prelude::*;
use bevy_ui_navigation::prelude::{FocusState, Focusable};

use super::*;

pub(crate) fn update_resolve_button(
    mut texts: Query<&mut Text>,
    button: Query<(&Children, &Buttons)>,
    targeting: Res<TargetingTypes>,
) {
    if targeting.is_changed() {
        for (children, buttons) in button.iter() {
            if buttons == &Buttons::Resolve {
                for child in children.iter() {
                    if let Ok(mut text) = texts.get_mut(*child) {
                        let label = match targeting.as_ref() {
                            TargetingTypes::SelectPower => "Resolve",
                            TargetingTypes::PowerTarget(_, _, _) => "De-Select",
                        };
                        if let Some(section) = text.sections.get_mut(0) {
                            section.value = label.to_string();
                        }
                    }
                }
            }
        }
    }
}

pub(crate) fn process_input(
    In(focused): In<Option<Entity>>,
    mut commands: Commands,
    interaction_query: TypedFocusedButtonQuery<'_, '_, '_, Buttons>,
    powers: Query<&Power>,
    targeting: Res<TargetingTypes>,
    power_containers: Query<Entity, With<PowerContainer>>,
    dice_pools: Query<&DicePool>,
) {
    let Some(focused) = focused else {
        return;
    };
    let Some((_, btn)) = interaction_query.get(focused).ok() else {
        return;
    };
    let power_targets =
        if let TargetingTypes::PowerTarget(targeting, power_entity, power) = targeting.as_ref() {
            Some((targeting, power_entity, power))
        } else {
            None
        };

    match btn {
        Buttons::Resolve => {
            if power_targets.is_some() {
                commands.insert_resource(TargetingTypes::SelectPower);
            } else {
                commands.insert_resource(NextState(Some(EncounterState::OutcomeResolution)));
            }
        }
        Buttons::Power(power_entity) => {
            if let Ok(power) = powers.get(*power_entity) {
                commands.insert_resource(TargetingTypes::PowerTarget(
                    power.targets(),
                    *power_entity,
                    *power,
                ));
            }
        }
        Buttons::Pool { pool, action } => {
            if let Some((PowerTargetingType::Single, power_entity, power)) = power_targets {
                let Ok(dice) = dice_pools.get(*pool) else {
                    return;
                };

                commands.insert_resource(TargetingTypes::SelectPower);
                commands.entity(*power_entity).despawn();
                commands.entity(*pool).despawn();

                commands
                    .entity(*action)
                    .insert(UpdatedDicePool)
                    .with_children(|p| {
                        for dice in power.apply(&[dice]).iter() {
                            p.spawn(*dice);
                        }
                    });
                for entity in power_containers.iter() {
                    commands.entity(entity).insert(UpdatePowers);
                }
            }
        }
        Buttons::Action(action) => {
            if let Some((PowerTargetingType::Action, power_entity, power)) = power_targets {
                commands.insert_resource(TargetingTypes::SelectPower);
                commands.entity(*power_entity).despawn();
                commands
                    .entity(*action)
                    .insert(UpdatedDicePool)
                    .with_children(|p| {
                        for dice in power.apply(&[]).iter() {
                            p.spawn(*dice);
                        }
                    });
                for entity in power_containers.iter() {
                    commands.entity(entity).insert(UpdatePowers);
                }
            }
        }
    }
}

pub(crate) fn update_current_focusables(
    mut buttons: Query<(&Buttons, &mut Focusable)>,
    targeting: Option<Res<TargetingTypes>>,
) {
    let Some(targeting) = targeting else {
        return;
    };

    match targeting.as_ref() {
        TargetingTypes::SelectPower => {
            for (button, mut focusable) in buttons.iter_mut() {
                let focus = matches!(button, Buttons::Resolve | Buttons::Power(_));
                let is_focusable = focusable.state() != FocusState::Blocked;
                if focus != is_focusable {
                    if focus {
                        focusable.unblock();
                    } else {
                        focusable.block();
                    }
                }
            }
        }
        TargetingTypes::PowerTarget(targeting_type, _, _) => match targeting_type {
            PowerTargetingType::Action => {
                for (button, mut focusable) in buttons.iter_mut() {
                    let focus = matches!(button, Buttons::Resolve | Buttons::Action(_));
                    let is_focusable = focusable.state() != FocusState::Blocked;
                    if focus != is_focusable {
                        if focus {
                            focusable.unblock();
                        } else {
                            focusable.block();
                        }
                    }
                }
            }
            _ => {
                for (button, mut focusable) in buttons.iter_mut() {
                    let focus = matches!(
                        button,
                        Buttons::Resolve | Buttons::Pool { pool: _, action: _ }
                    );
                    let is_focusable = focusable.state() != FocusState::Blocked;
                    if focus != is_focusable {
                        if focus {
                            focusable.unblock();
                        } else {
                            focusable.block();
                        }
                    }
                }
            }
        },
    };
}

pub(super) fn update_powers(
    mut commands: Commands,
    power_containers: Query<Entity, With<UpdatePowers>>,
    powers: Query<(Entity, &Power)>,
    assets: Res<MainGameAssets>,
    asset_server: Res<AssetServer>,
) {
    let mut power_buttons = Vec::new();
    for container in power_containers.iter() {
        info!("Updating Powers");
        let root = root(powers_container.nb(), &asset_server, &mut commands, |p| {
            for (entity, power) in powers.iter() {
                power_buttons.push((
                    focus_button(power_card_container.nb(), apply_power_card_state, p, |p| {
                        power.display_bundle(&assets, 50., p);
                    }),
                    entity,
                ));
            }
        });
        commands
            .entity(container)
            .despawn_descendants()
            .add_child(root);
    }

    for (button, power) in power_buttons.iter() {
        info!("Adding button power");
        commands.entity(*button).insert(Buttons::Power(*power));
    }
}

pub(super) fn clear_updated_powers(
    mut commands: Commands,
    power_containers: Query<Entity, With<UpdatePowers>>,
) {
    for entity in power_containers.iter() {
        info!("Clear Powers");
        commands.entity(entity).remove::<UpdatePowers>();
    }
}
