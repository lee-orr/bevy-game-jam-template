use bevy::prelude::*;

use bevy_ui_dsl::*;

use crate::{
    app_state::AppState,
    ui::{
        buttons::{focus_text_button, focused_button_activated, TypedFocusedButtonQuery},
        classes::*,
        intermediary_node_bundles::*,
    },
};

use super::game_state::GameState;
use dexterous_developer::{
    dexterous_developer_setup, ReloadableApp, ReloadableAppContents, ReloadableElementsSetup,
};
pub struct GameCompletedPlugin;

impl Plugin for GameCompletedPlugin {
    fn build(&self, app: &mut App) {
        app.setup_reloadable_elements::<reloadable>();
    }
}

#[dexterous_developer_setup(game_completed)]
fn reloadable(app: &mut ReloadableAppContents) {
    app.reset_setup_in_state::<Screen, _, _>(GameState::Complete, setup)
        .add_systems(
            Update,
            (
                process_keyboard_input,
                (focused_button_activated.pipe(process_input)),
            )
                .run_if(in_state(GameState::Complete)),
        );
}

#[derive(Component)]
struct Screen;

#[derive(Component)]
struct Button;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut menu_button = None;
    let r = root((overlay, c_root), &asset_server, &mut commands, |p| {
        node(primary_box, p, |p| {
            node((span.nb(), primary_box_main.nb()), p, |p| {
                text("Game", (), main_text, p);
                text("Completed!", (), main_text, p);
            });
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
    commands.entity(menu_button.unwrap()).insert(Button);
}

fn exit(mut commands: Commands, query: Query<Entity, With<Screen>>) {
    for item in query.iter() {
        commands.entity(item).despawn_recursive();
    }
}

fn process_input(
    In(focused): In<Option<Entity>>,
    mut commands: Commands,
    interaction_query: TypedFocusedButtonQuery<'_, '_, '_, Button>,
) {
    let Some(focused) = focused else {
        return;
    };
    let Some((_entity, _btn)) = interaction_query.get(focused).ok() else {
        return;
    };
    commands.insert_resource(NextState(Some(AppState::MainMenu)));
}

fn process_keyboard_input(mut commands: Commands, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        commands.insert_resource(NextState(Some(AppState::MainMenu)));
    }
}
