use bevy::prelude::*;
use bevy_ui_dsl::*;

use crate::{
    app_state::AppState,
    assets::MainGameAssets,
    ui::{
        buttons::{focus_text_button, focused_button_activated, TypedFocusedButtonQuery},
        classes::*,
        colors::SCREEN_BACKGROUND_COLOR,
        intermediary_node_bundles::*,
    },
};
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup)
            .add_systems(OnExit(AppState::MainMenu), exit)
            .add_systems(
                Update,
                (focused_button_activated.pipe(process_input)).run_if(in_state(AppState::MainMenu)),
            );
    }
}

#[derive(Component)]
struct Screen;

#[derive(Component)]
enum Buttons {
    Start,
    Credits,
}

fn setup(mut commands: Commands, _assets: Res<MainGameAssets>, asset_server: Res<AssetServer>) {
    commands.insert_resource(ClearColor(SCREEN_BACKGROUND_COLOR));

    let mut start_button = None;
    let mut credits_button = None;

    let r = root(c_root, &asset_server, &mut commands, |p| {
        node(primary_box, p, |p| {
            node((span.nb(), primary_box_main.nb(), centered.nb()), p, |p| {
                text(
                    "The Just",
                    (),
                    (main_text, knight_text, knight_text_color),
                    p,
                );
                text("Two", (), (main_text, druid_text, druid_text_color), p);
            });
            focus_text_button(
                "Start Game",
                (c_button.nb(), primary_box_item.nb()),
                apply_button_state,
                button_text,
                p,
            )
            .set(&mut start_button);
            focus_text_button(
                "Credits",
                (c_button.nb(), primary_box_item.nb()),
                apply_button_state,
                button_text,
                p,
            )
            .set(&mut credits_button);
        });
    });
    commands.entity(r).insert(Screen);
    commands
        .entity(start_button.unwrap())
        .insert(Buttons::Start);
    commands
        .entity(credits_button.unwrap())
        .insert(Buttons::Credits);
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
) {
    let Some(focused) = focused else {
        return;
    };
    let Some((_entity, btn)) = interaction_query.get(focused).ok() else {
        return;
    };
    match btn {
        Buttons::Start => commands.insert_resource(NextState(Some(AppState::InGame))),
        Buttons::Credits => commands.insert_resource(NextState(Some(AppState::Credits))),
    };
}
