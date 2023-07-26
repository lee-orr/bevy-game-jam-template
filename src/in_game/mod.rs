pub mod encounter;
mod factions;
mod game_completed;
mod game_over;
mod game_state;
pub mod mission;
mod pause_screen;
pub mod story;
mod world_map;

use bevy::{
    audio::{Volume, VolumeLevel},
    ecs::schedule::ScheduleLabel,
    input::common_conditions::input_toggle_active,
    prelude::*,
};
use bevy_inspector_egui::quick::StateInspectorPlugin;

use crate::{app_state::AppState, assets::MainGameAssets};

use self::{
    encounter::{dice_pools, powers::Power, sequencing::EncounterState, EncounterPlugin},
    game_completed::GameCompletedPlugin,
    game_over::GameOverPlugin,
    game_state::{GameState, PauseState},
    mission::MissionPlugin,
    pause_screen::PausePlugin,
    story::StoryPlugin,
    world_map::WorldMapPlugin,
};

pub use self::encounter::{Challengers, Locations, Players};
pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PausePlugin,
            EncounterPlugin,
            WorldMapPlugin,
            StoryPlugin,
            MissionPlugin,
            GameOverPlugin,
            GameCompletedPlugin,
        ))
        .add_state::<GameState>()
        .register_type::<GameState>()
        .add_plugins(
            StateInspectorPlugin::<GameState>::default()
                .run_if(input_toggle_active(false, KeyCode::F1)),
        )
        .add_systems(OnEnter(AppState::InGame), setup)
        .add_systems(OnExit(AppState::InGame), (exit, clear_audio))
        .add_systems(Update, (enable_audio).run_if(in_state(AppState::InGame)))
        .add_systems(
            Update,
            run_in_game_update.run_if(in_state(PauseState::None)),
        );
    }
}

#[derive(Component)]
struct InGame;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct InGameUpdate;

fn setup(mut commands: Commands, assets: Res<MainGameAssets>) {
    commands.insert_resource(NextState(Some(GameState::WorldMap)));
    commands.insert_resource(AmbientLight {
        color: Color::rgba_u8(32, 20, 19, 255),
        brightness: 0.02,
    });

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

            p.spawn(Power::SplitDice);
            p.spawn(Power::Advantage);
            p.spawn(Power::StaticBonus(2));
            p.spawn(Power::AddDice(dice_pools::DiceType::D3));
        });
}

fn exit(mut commands: Commands, query: Query<Entity, With<InGame>>) {
    commands.insert_resource(NextState(Some(GameState::None)));
    commands.insert_resource(NextState(Some(PauseState::None)));
    commands.insert_resource(NextState(Some(EncounterState::None)));
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
