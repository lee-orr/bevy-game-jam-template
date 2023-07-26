mod app_state;
mod assets;
mod in_game;
mod menus;
mod toon_material;
mod ui;

use std::time::Duration;

use app_state::AppState;
use assets::{MainGameAssetPlugin, MainGameAssets};
use bevy::{
    asset::ChangeWatcher,
    core_pipeline::{clear_color::ClearColorConfig, tonemapping::Tonemapping},
    input::common_conditions::input_toggle_active,
    prelude::*,
};

use bevy_inspector_egui::quick::{StateInspectorPlugin, WorldInspectorPlugin};
use bevy_turborand::prelude::RngPlugin;
use bevy_vector_shapes::Shape2dPlugin;
use credits::CreditsPlugin;
use in_game::InGamePlugin;
use loading_state::LoadingScreenPlugin;
use menu::MainMenuPlugin;
use menus::{credits, loading_state, menu};

use ui::{colors::DEFAULT_AMBIENT, UiPlugin};

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_secs_f32(0.5)),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
            Shape2dPlugin::default(),
            WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::F1)),
            RngPlugin::default(),
        ))
        .insert_resource(ClearColor(ui::colors::SCREEN_BACKGROUND_COLOR))
        .insert_resource(DEFAULT_AMBIENT)
        .add_plugins((
            LoadingScreenPlugin,
            MainMenuPlugin,
            CreditsPlugin,
            InGamePlugin,
            MainGameAssetPlugin,
            UiPlugin,
        ))
        .add_state::<AppState>()
        .register_type::<AppState>()
        .register_type::<MainGameAssets>()
        .add_plugins(
            StateInspectorPlugin::<AppState>::default()
                .run_if(input_toggle_active(false, KeyCode::F1)),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, fix_light)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(-2., 5., -5.))
            .looking_at(Vec3::Y, Vec3::Y),
        tonemapping: Tonemapping::AcesFitted,
        ..default()
    });

    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 1,
            ..default()
        },
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::None,
        },
        tonemapping: Tonemapping::AcesFitted,
        ..default()
    });
}

fn fix_light(
    mut directional: Query<&mut DirectionalLight, Added<DirectionalLight>>,
    mut point: Query<&mut PointLight, Added<PointLight>>,
) {
    for mut light in directional.iter_mut() {
        if !light.shadows_enabled {
            light.shadows_enabled = true;
        }
    }
    for mut light in point.iter_mut() {
        if !light.shadows_enabled {
            light.shadows_enabled = true;
        }
    }
}
