mod game_completed;
mod game_over;
mod game_state;
mod pause_screen;

use bevy::{
    audio::{Volume, VolumeLevel},
    ecs::schedule::ScheduleLabel,
    input::common_conditions::input_toggle_active,
    prelude::*,
};
use bevy_inspector_egui::quick::StateInspectorPlugin;

use crate::{
    app_state::AppState,
    assets::MainGameAssets,
    ui::colors::{DEFAULT_AMBIENT, DEFAULT_CLEAR},
};

use self::{
    game_completed::GameCompletedPlugin,
    game_over::GameOverPlugin,
    game_state::{GameState, PauseState},
    pause_screen::PausePlugin,
};
use dexterous_developer::{
    dexterous_developer_setup, ReloadableApp, ReloadableAppContents, ReloadableElementsSetup,
};
pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PausePlugin, GameOverPlugin, GameCompletedPlugin))
            .add_state::<GameState>()
            .register_type::<GameState>()
            .add_plugins(
                StateInspectorPlugin::<GameState>::default()
                    .run_if(input_toggle_active(false, KeyCode::F1)),
            )
            .add_systems(OnExit(AppState::InGame), (exit, clear_audio))
            .add_systems(Update, (enable_audio).run_if(in_state(AppState::InGame)))
            .setup_reloadable_elements::<reloadable>();
    }
}

#[dexterous_developer_setup(in_game)]
fn reloadable(app: &mut ReloadableAppContents) {
    app.reset_setup_in_state::<InGame, _, _>(AppState::InGame, setup)
        .add_systems(
            Update,
            run_in_game_update.run_if(in_state(PauseState::None)),
        );
}

#[derive(Component)]
struct InGame;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct InGameUpdate;

fn setup(mut commands: Commands, assets: Res<MainGameAssets>) {
    commands.insert_resource(ClearColor(DEFAULT_CLEAR));
    commands.insert_resource(DEFAULT_AMBIENT);
    commands
        .spawn((
            InGame,
            TransformBundle::default(),
            VisibilityBundle::default(),
        ))
        .with_children(|p| {
            p.spawn(AudioBundle {
                source: assets.menu_music.clone(),
                settings: PlaybackSettings {
                    paused: true,
                    volume: Volume::Absolute(VolumeLevel::new(0.)),
                    ..Default::default()
                },
            });
        });
}

fn exit(mut commands: Commands, query: Query<Entity, With<InGame>>) {
    commands.insert_resource(NextState(Some(GameState::None)));
    commands.insert_resource(NextState(Some(PauseState::None)));
    for item in query.iter() {
        commands.entity(item).despawn_recursive();
    }
}

fn clear_audio(audio: Query<&AudioSink>) {
    for audio in audio.iter() {
        audio.stop();
    }
}

fn enable_audio(audio: Query<&AudioSink>) {
    for audio in audio.iter() {
        if audio.is_paused() {
            audio.play();
        }
    }
}

fn run_in_game_update(world: &mut World) {
    let _ = world.try_run_schedule(InGameUpdate);
}
