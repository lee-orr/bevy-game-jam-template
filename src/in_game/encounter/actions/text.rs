use bevy::prelude::*;
use bevy_ui_dsl::*;

use crate::{
    in_game::{
        encounter::{action_resolutions::ActiveResolution, sequencing::EncounterState},
        InGameUpdate,
    },
    ui::{
        buttons::{focus_text_button, focused_button_activated},
        classes::*,
        intermediary_node_bundles::IntoIntermediaryNodeBundle,
    },
};

use super::{ActionChoice, ActionType, Resolution};

pub struct TextActionPlugin;

impl Plugin for TextActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            display_text_resolution.run_if(in_state(EncounterState::OutcomeResolution)),
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

fn display_text_resolution(
    mut commands: Commands,
    resolution: Query<(Entity, &ActionChoice, &Resolution, &ActionType), Added<ActiveResolution>>,
    asset_server: Res<AssetServer>,
) {
    let Ok((_entity, choice, resolution, action_type)) = resolution.get_single() else {
        return;
    };
    if !matches!(action_type, ActionType::Text) {
        return;
    }
    let mut next_button = None;
    let root = root(c_root, &asset_server, &mut commands, |p| {
        node(primary_box, p, |p| {
            node((span.nb(), primary_box_main.nb()), p, |p| {
                text(&choice.title, (), (main_text, knight_text), p);
            });
            text(
                match resolution.result {
                    super::ActionResult::CriticalFail => "Failed Badly",
                    super::ActionResult::Fail => "Failed",
                    super::ActionResult::Success => "Succeeded!",
                    super::ActionResult::CriticalSuccess => "Amazing Success!",
                },
                primary_box_item.nb(),
                standard_text,
                p,
            );
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
