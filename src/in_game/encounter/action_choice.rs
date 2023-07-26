use crate::{
    assets::MainGameAssets,
    in_game::InGameUpdate,
    ui::{
        buttons::{focus_button, focused_button_activated, TypedFocusedButtonQuery},
        classes::*,
        intermediary_node_bundles::*,
        DisplayBundle,
    },
};
use bevy::prelude::*;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use bevy_ui_dsl::*;

use super::sequencing::{
    ClearUnusuedActions, EncounterState, FlushAvailableActions, PublishAvailableActions,
};

use super::actions::*;

pub struct ActionChoicePlugin;

impl Plugin for ActionChoicePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ChoiceButton>()
            .add_systems(
                OnEnter(EncounterState::ActionChoice),
                (
                    clear_unused_actions,
                    apply_deferred
                        .in_set(ClearUnusuedActions)
                        .before(PublishAvailableActions),
                    apply_deferred
                        .in_set(FlushAvailableActions)
                        .after(PublishAvailableActions),
                    setup,
                )
                    .chain(),
            )
            .add_systems(OnExit(EncounterState::ActionChoice), exit)
            .add_systems(
                InGameUpdate,
                (focused_button_activated.pipe(process_input))
                    .run_if(in_state(EncounterState::ActionChoice)),
            );
    }
}

#[derive(Component)]
struct Screen;

#[derive(Component, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
struct ChoiceButton(Entity);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    assets: Res<MainGameAssets>,
    actions: Query<(Entity, &ActionChoice), Without<ChallengerAction>>,
) {
    let mut choices = vec![];
    let r = root(c_action_choice_root, &asset_server, &mut commands, |p| {
        for (action_entity, choice) in actions.iter() {
            let button = focus_button(card.nb(), apply_card_state, p, |p| {
                node(card_title.nb(), p, |p| {
                    text(choice.title.as_str(), (), (card_title_text, druid_text), p);
                });
                node(card_content.nb(), p, |p| {
                    text(choice.content.as_str(), (), standard_text, p);
                });
                node(card_footer.nb(), p, |p| {
                    node(
                        card_fail.nb(),
                        p,
                        |p: &mut UiChildBuilder<'_, '_, '_, '_>| {
                            text(
                                format!("{}", choice.fail),
                                (),
                                (card_fail_text, druid_text),
                                p,
                            );
                        },
                    );
                    node(card_dice.nb(), p, |p| {
                        for dice in choice.dice_pool.iter() {
                            dice.display_bundle(&assets, 40., p);
                        }
                    });
                    node(card_success.nb(), p, |p| {
                        text(
                            format!("{}", choice.critical_success),
                            (),
                            (card_critical, druid_text),
                            p,
                        );
                        text(
                            format!("{}", choice.success),
                            (),
                            (card_success_text, druid_text),
                            p,
                        );
                    });
                });
            });
            choices.push((button, action_entity));
        }
    });
    commands.entity(r).insert(Screen);
    for (button, action_entity) in choices.iter() {
        commands
            .entity(*button)
            .insert(ChoiceButton(*action_entity));
    }
}

type UnchosenActions<'w, 's> = Query<
    'w,
    's,
    Entity,
    (
        With<ActionChoice>,
        Without<ChosenAction>,
        Without<ChallengerAction>,
    ),
>;

fn exit(
    mut commands: Commands,
    query: Query<Entity, With<Screen>>,
    unchosen_actions: UnchosenActions,
) {
    for item in query.iter() {
        commands.entity(item).despawn_recursive();
    }
    for item in unchosen_actions.iter() {
        commands.entity(item).despawn_recursive();
    }
}

fn clear_unused_actions(mut commands: Commands, actions: Query<Entity, With<ActionChoice>>) {
    for item in actions.iter() {
        commands.entity(item).despawn_recursive();
    }
}

fn process_input(
    In(focused): In<Option<Entity>>,
    mut commands: Commands,
    interaction_query: TypedFocusedButtonQuery<'_, '_, '_, ChoiceButton>,
) {
    let Some(focused) = focused else {
        return;
    };
    let Some((_entity, btn)) = interaction_query.get(focused).ok() else {
        return;
    };
    commands.entity(btn.0).insert(ChosenAction);
    commands.insert_resource(NextState(Some(EncounterState::ProbabilitySetup)));
}
