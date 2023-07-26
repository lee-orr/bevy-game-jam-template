use bevy::prelude::*;
use bevy_asset_loader::prelude::{AssetCollection, LoadingState, LoadingStateAppExt};
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

use crate::{app_state::AppState, menus::credits::Credits};

pub struct MainGameAssetPlugin;

impl Plugin for MainGameAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::LoadingMenu).continue_to_state(AppState::MainMenu),
        )
        .add_collection_to_loading_state::<_, MainGameAssets>(AppState::LoadingMenu);
    }
}

#[derive(AssetCollection, Resource, Default, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct MainGameAssets {
    #[asset(path = "music/test.flac")]
    pub menu_music: Handle<AudioSource>,
    #[asset(path = "fonts/AMERSN__.ttf")]
    pub default_font: Handle<Font>,

    #[asset(path = "credits.cr.yaml")]
    pub credits: Handle<Credits>,
}
