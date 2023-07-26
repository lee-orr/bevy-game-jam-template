use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::StateInspectorPlugin;
use bevy_ui_dsl::*;

use crate::{
    app_state::AppState,
    ui::{
        buttons::{focus_text_button, focused_button_activated, TypedFocusedButtonQuery},
        classes::*,
        intermediary_node_bundles::*,
    },
};

use super::game_state::PauseState;
pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<PauseState>()
            .register_type::<PauseState>()
            .add_plugins(
                StateInspectorPlugin::<PauseState>::default()
                    .run_if(input_toggle_active(false, KeyCode::F1)),
            )
            .add_systems(OnEnter(PauseState::Paused), setup)
            .add_systems(OnExit(PauseState::Paused), exit)
            .add_systems(
                Update,
                (
                    process_keyboard_input,
                    (focused_button_activated.pipe(process_input)),
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

#[derive(Component)]
struct Screen;

#[derive(Component)]
enum Buttons {
    Resume,
    Menu,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut resume_button = None;
    let mut menu_button = None;
    let r = root((overlay, c_root), &asset_server, &mut commands, |p| {
        node(primary_box, p, |p| {
            node((span.nb(), primary_box_main.nb()), p, |p| {
                text("Game", (), main_text, p);
                text("Paused", (), main_text, p);
            });
            focus_text_button(
                "Resume Game",
                (c_button.nb(), primary_box_item.nb()),
                apply_button_state,
                button_text,
                p,
            )
            .set(&mut resume_button);
            focus_text_button(
                "Main Menu",
                (c_button.nb(), primary_box_item.nb()),
                apply_button_state,
                button_text,
                p,
            )
            .set(&mut menu_button);
        });
    });
    commands.entity(r).insert(Screen);
    commands
        .entity(resume_button.unwrap())
        .insert(Buttons::Resume);
    commands.entity(menu_button.unwrap()).insert(Buttons::Menu);
}

fn exit(mut commands: Commands, query: Query<Entity, With<Screen>>) {
    for item in query.iter() {
        commands.entity(item).despawn_recursive();
    }
}

fn process_input(
    In(focused): In<Option<Entity>>,
    mut commands: Commands,
    interaction_query: TypedFocusedButtonQuery<'_, '_, '_, Buttons>,
    paused: Res<State<PauseState>>,
) {
    let Some(focused) = focused else {
        return;
    };
    let Some((_entity, btn)) = interaction_query.get(focused).ok() else {
        return;
    };
    match btn {
        Buttons::Resume => commands.insert_resource(NextState(Some(match paused.get() {
            PauseState::None => PauseState::Paused,
            PauseState::Paused => PauseState::None,
        }))),
        Buttons::Menu => commands.insert_resource(NextState(Some(AppState::MainMenu))),
    };
}

fn process_keyboard_input(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    paused: Res<State<PauseState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        commands.insert_resource(NextState(Some(match paused.get() {
            PauseState::None => PauseState::Paused,
            PauseState::Paused => PauseState::None,
        })));
    }
}
