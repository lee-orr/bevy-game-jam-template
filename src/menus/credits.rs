use bevy::{prelude::*, reflect::TypeUuid};
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy_inspector_egui::InspectorOptions;
use bevy_ui_dsl::*;
use dexterous_developer::{
    dexterous_developer_setup, ReloadableApp, ReloadableAppContents, ReloadableElementsSetup,
};
use serde::Deserialize;

use crate::{
    app_state::AppState,
    assets::MainGameAssets,
    ui::{
        buttons::{focus_text_button, focused_button_activated},
        classes::*,
        colors::SCREEN_BACKGROUND_COLOR,
        intermediary_node_bundles::*,
    },
};

use super::game_title;
pub struct CreditsPlugin;

impl Plugin for CreditsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Credits>()
            .register_asset_reflect::<Credits>()
            .add_plugins(YamlAssetPlugin::<Credits>::new(&["cr.yaml"]))
            .setup_reloadable_elements::<reloadable>();
    }
}

#[dexterous_developer_setup(credits)]
fn reloadable(app: &mut ReloadableAppContents) {
    app.reset_setup_in_state::<Screen, _, _>(AppState::Credits, setup)
        .add_systems(
            Update,
            (focused_button_activated.pipe(process_input)).run_if(in_state(AppState::Credits)),
        );
}

#[derive(Component)]
struct Screen;

#[derive(Reflect, InspectorOptions, Deserialize, TypeUuid, Default)]
#[uuid = "cdac5b3f-215c-4728-afe4-897f514ecf42"]

pub struct Credits(Vec<String>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    assets: Res<MainGameAssets>,
    credits: Res<Assets<Credits>>,
) {
    let credits = credits
        .get(&assets.credits)
        .map(|v| v.0.as_slice())
        .unwrap_or(&[]);
    commands.insert_resource(ClearColor(SCREEN_BACKGROUND_COLOR));

    let r = root((c_root, opaque.nb()), &asset_server, &mut commands, |p| {
        node(primary_box, p, |p| {
            game_title::game_title(p);

            for credit in credits.iter() {
                text(credit.as_str(), primary_box_item.nb(), standard_text, p);
            }

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

fn process_input(In(focused): In<Option<Entity>>, mut commands: Commands) {
    let Some(_) = focused else {
        return;
    };
    commands.insert_resource(NextState(Some(AppState::MainMenu)));
}
