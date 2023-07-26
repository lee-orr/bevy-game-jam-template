use bevy::{prelude::*, reflect::TypeUuid};
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use bevy_turborand::{DelegatedRng, GlobalRng, TurboRand};
use serde::Deserialize;

use crate::{app_state::AppState, assets::MainGameAssets};

use super::game_state::GameState;

pub struct StoryPlugin;

impl Plugin for StoryPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Story>()
            .register_type::<Phase>()
            .register_type::<PhaseRound>()
            .add_plugins(YamlAssetPlugin::<Story>::new(&["st.yaml"]))
            .add_systems(OnEnter(AppState::InGame), start_story)
            .add_systems(Update, check_phase.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Resource, Default, Reflect, Deserialize, InspectorOptions, Clone)]
#[reflect(Resource, InspectorOptions)]
pub struct PhaseRound(pub usize, pub usize);

#[derive(Resource, Default, Reflect, Deserialize, InspectorOptions, Clone)]
#[reflect(Resource, InspectorOptions)]
pub struct Phase {
    pub min_missions: usize,
    pub max_missions: usize,
    pub simulatneous_missions: usize,
    pub missions: Vec<String>,
}

#[derive(Resource, Default, Reflect, InspectorOptions, Deserialize, TypeUuid, Clone)]
#[reflect(Resource, InspectorOptions)]
#[uuid = "bf4f4ba3-b7bd-4954-b51f-011455c7ff0d"]
pub struct Story {
    pub title: String,
    pub phases: Vec<Phase>,
}

fn start_story(mut commands: Commands, assets: Res<MainGameAssets>, stories: Res<Assets<Story>>) {
    let Some(story) = stories.get(&assets.story) else {
        return;
    };
    let story = story.clone();
    let Some(phase) = story.phases.first() else {
        return;
    };
    let phase = phase.clone();

    commands.insert_resource(story);
    commands.insert_resource(phase);
    commands.insert_resource(PhaseRound::default());
}

fn check_phase(
    mut commands: Commands,
    round: Res<PhaseRound>,
    phase: Res<Phase>,
    story: Res<Story>,
    mut global_rng: ResMut<GlobalRng>,
) {
    if !round.is_changed() {
        return;
    }
    if round.0 < phase.min_missions {
        return;
    }
    let rng = global_rng.get_mut();
    if round.0 >= phase.max_missions || rng.bool() {
        let next_phase = round.1 + 1;
        if let Some(phase) = story.phases.get(next_phase) {
            commands.insert_resource(phase.clone());
            commands.insert_resource(PhaseRound(0, next_phase));
        } else {
            commands.insert_resource(NextState(Some(GameState::Complete)));
        }
    }
}
