use bevy::prelude::*;
use bevy_asset_loader::prelude::{AssetCollection, LoadingState, LoadingStateAppExt};
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

use crate::{
    app_state::AppState,
    in_game::{
        encounter::encounter_setup_types::Encounters, mission::mission_types::Missions,
        story::Story, Challengers, Locations, Players,
    },
};

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
    #[asset(path = "textures/shadow-gradient.png")]
    pub shadow_gradient: Handle<Image>,
    #[asset(texture_atlas(
        tile_size_x = 512.,
        tile_size_y = 512.,
        columns = 4,
        rows = 4,
        padding_x = 0.,
        padding_y = 0.,
        offset_x = 0.,
        offset_y = 0.
    ))]
    #[asset(path = "textures/icons.png")]
    pub icons: Handle<TextureAtlas>,

    #[asset(path = "music/test.flac")]
    pub menu_music: Handle<AudioSource>,

    #[asset(path = "fonts/ENDOR___.ttf")]
    pub knights_font: Handle<Font>,
    #[asset(path = "fonts/IMMORTAL.ttf")]
    pub druids_font: Handle<Font>,
    #[asset(path = "fonts/AMERSN__.ttf")]
    pub default_font: Handle<Font>,
    #[asset(path = "challengers.ch.yaml")]
    pub challengers: Handle<Challengers>,

    #[asset(path = "locations.lc.yaml")]
    pub locations: Handle<Locations>,
    #[asset(path = "players.pl.yaml")]
    pub players: Handle<Players>,
    #[asset(path = "story.st.yaml")]
    pub story: Handle<Story>,
    #[asset(path = "missions.ms.yaml")]
    pub missions: Handle<Missions>,
    #[asset(path = "encounters.en.yaml")]
    pub encounters: Handle<Encounters>,

    #[asset(path = "textures/color-pallet-labelled.png")]
    pub default_color_pallet: Handle<Image>,
    #[asset(path = "models/world-map.gltf#Scene0")]
    pub world_map: Handle<Scene>,
}
