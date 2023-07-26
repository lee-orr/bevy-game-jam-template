use bevy::prelude::*;
use bevy_ui_dsl::*;

use crate::{
    app_state::AppState,
    ui::{
        buttons::{focus_text_button, focused_button_activated},
        classes::*,
        colors::SCREEN_BACKGROUND_COLOR,
        intermediary_node_bundles::*,
    },
};
pub struct CreditsPlugin;

impl Plugin for CreditsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Credits), setup)
            .add_systems(OnExit(AppState::Credits), exit)
            .add_systems(
                Update,
                (focused_button_activated.pipe(process_input)).run_if(in_state(AppState::Credits)),
            );
    }
}

#[derive(Component)]
struct Screen;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ClearColor(SCREEN_BACKGROUND_COLOR));

    let r = root(c_root, &asset_server, &mut commands, |p| {
        node(primary_box, p, |p| {
            node((span.nb(), primary_box_main.nb()), p, |p| {
                text(
                    "The Just",
                    (),
                    (main_text, knight_text, knight_text_color),
                    p,
                );
                text("Two", (), (main_text, druid_text, druid_text_color), p);
            });
            text("by Lee-Orr", primary_box_item.nb(), standard_text, p);
            text(
                "Built using the Bevy Game Engine",
                primary_box_item.nb(),
                standard_text,
                p,
            );
            text(
                "Fonts by Appostrophic Labs, sourced from 1001freefonts.com",
                primary_box_item.nb(),
                standard_text,
                p,
            );
            text(
                "All other artistic assets created by Lee-Orr",
                primary_box_item.nb(),
                standard_text,
                p,
            );
            focus_text_button(
                "Main Menu",
                (c_button.nb(), primary_box_item.nb()),
                apply_button_state,
                button_text,
                p,
            );
        });
    });
    commands.entity(r).insert(Screen);
}

fn exit(mut commands: Commands, query: Query<Entity, With<Screen>>) {
    for item in query.iter() {
        commands.entity(item).despawn_recursive();
    }
}

fn process_input(In(focused): In<Option<Entity>>, mut commands: Commands) {
    let Some(_) = focused else {
        return;
    };
    commands.insert_resource(NextState(Some(AppState::MainMenu)));
}
