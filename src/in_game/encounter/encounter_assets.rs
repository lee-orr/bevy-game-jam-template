use std::f32::consts::PI;

use crate::{
    assets::MainGameAssets,
    materialized_scene::{MaterializedScene, MaterializedSceneBundle, MaterializedSceneReference},
    toon_material::{create_material_with_pallet, ToonMaterial},
};

use super::{
    encounter_setup_types::{self},
    sequencing::EncounterState,
};
use bevy::{
    gltf::{Gltf, GltfNode},
    prelude::*,
    utils::{HashMap, HashSet},
};
use bevy_asset_loader::prelude::{
    AssetCollection, DynamicAssets, LoadingState, LoadingStateAppExt, StandardDynamicAsset,
};
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

pub struct EncounterAssetPlugin;

impl Plugin for EncounterAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(EncounterState::Loading)
                .continue_to_state(EncounterState::Introduction),
        )
        .add_collection_to_loading_state::<_, EncounterAssets>(EncounterState::Loading)
        .add_systems(OnExit(EncounterState::Loading), setup_materials)
        .add_systems(OnEnter(EncounterState::None), unload_assets);
    }
}

#[derive(AssetCollection, Resource, Default, Reflect, InspectorOptions, Debug)]
#[reflect(Resource, InspectorOptions)]
pub struct EncounterAssets {
    #[asset(key = "scenes", collection(typed, mapped))]
    pub scenes: HashMap<String, Handle<Gltf>>,
    #[asset(key = "color_pallets", collection(typed, mapped))]
    pub color_pallets: HashMap<String, Handle<Image>>,
}

#[derive(Resource, Default, Reflect, InspectorOptions, Debug)]
#[reflect(Resource, InspectorOptions)]
pub struct Materials(HashMap<String, Handle<ToonMaterial>>);

fn setup_materials(
    mut commands: Commands,
    assets: Res<EncounterAssets>,
    main_assets: Res<MainGameAssets>,
    mut materials: ResMut<Assets<ToonMaterial>>,
) {
    let materials = assets
        .color_pallets
        .iter()
        .map(|(name, color_texture)| {
            (
                name.clone(),
                materials.add(create_material_with_pallet(
                    color_texture,
                    main_assets.as_ref(),
                )),
            )
        })
        .collect();
    commands.insert_resource(Materials(materials));
}

fn unload_assets(mut commands: Commands) {
    info!("Unloading Encounter Assets");
    commands.remove_resource::<EncounterAssets>();
}

pub fn setup_encounter_assets(
    setup: &encounter_setup_types::EncounterSetup,
    dynamic: &mut DynamicAssets,
) {
    info!("Setting Up Encounter Assets");
    let mut scene_refs = setup
        .challengers
        .iter()
        .map(|(_, c)| c.scene.clone())
        .collect::<Vec<_>>();
    if let Some(loc) = &setup.location {
        scene_refs.push(loc.scene.clone());
    }
    if let Some(p) = &setup.player {
        scene_refs.push(p.scene.clone());
    }

    let scenes = scene_refs
        .iter()
        .map(|v| v.gltf.clone())
        .collect::<HashSet<_>>();
    let color_pallets = scene_refs
        .iter()
        .flat_map(|v| v.pallet.clone())
        .collect::<HashSet<_>>();

    info!("Encounter Scenes {scenes:?}");
    info!("Encounter Pallets {color_pallets:?}");

    dynamic.register_asset(
        "scenes",
        Box::new(StandardDynamicAsset::Files {
            paths: scenes.into_iter().collect(),
        }),
    );
    dynamic.register_asset(
        "color_pallets",
        Box::new(StandardDynamicAsset::Files {
            paths: color_pallets.into_iter().collect(),
        }),
    );
}

pub struct SceneBundler<'a>(
    &'a EncounterAssets,
    &'a Materials,
    &'a Assets<Gltf>,
    &'a Assets<GltfNode>,
);

impl<'a> SceneBundler<'a> {
    pub fn new(
        assets: &'a EncounterAssets,
        mats: &'a Materials,
        gltf: &'a Assets<Gltf>,
        gltf_node: &'a Assets<GltfNode>,
    ) -> Self {
        info!("Setting up bundler");
        info!("Assets: {assets:?}");
        info!("Materials: {mats:?}");
        Self(assets, mats, gltf, gltf_node)
    }

    pub fn scene(&self, reference: &MaterializedSceneReference) -> Option<MaterializedSceneBundle> {
        info!("Getting Scene Reference {reference:?}");
        let gltf = self.0.scenes.get(&reference.gltf)?;
        info!("Got GLTF Handle");
        let gltf = self.2.get(gltf)?;
        info!(
            "Loaded GLTF Reference - contains scenes {:?}",
            &gltf.named_scenes
        );
        let scene = gltf.named_scenes.get(&reference.scene)?;
        info!("got scene handle");
        let pallet = &reference.pallet.get(0)?;
        info!("got pallet name {pallet:?}");
        let pallet = self.1 .0.get(pallet.as_str())?;
        info!("got pallet handle");
        Some(MaterializedSceneBundle {
            spawner: MaterializedScene {
                scene: scene.clone(),
                material: pallet.clone(),
            },
            ..Default::default()
        })
    }

    pub fn camera_position(&self, reference: &MaterializedSceneReference) -> Option<Transform> {
        info!("Getting Canera Reference {reference:?}");
        let gltf = self.0.scenes.get(&reference.gltf)?;
        info!("Got GLTF Handle");
        let gltf = self.2.get(gltf)?;

        info!("got gltf");
        let camera_node = gltf.named_nodes.get("camera_pos")?;
        info!("found camera node");
        let camera_node = self.3.get(camera_node)?;
        info!("got camera transform");
        let mut transform = camera_node.transform;
        transform.rotate_local_x(-1. * PI / 2.);
        Some(transform)
    }

    pub fn player_position(&self, reference: &MaterializedSceneReference) -> Option<Transform> {
        info!("Getting Player Reference {reference:?}");
        let gltf = self.0.scenes.get(&reference.gltf)?;
        info!("Got GLTF Handle");
        let gltf = self.2.get(gltf)?;

        info!("got gltf");
        let player_node = gltf.named_nodes.get("player_pos")?;
        info!("found player node");
        let player_node = self.3.get(player_node)?;
        info!("got player transform");
        let mut transform = player_node.transform;
        transform.rotate_local_x(-1. * PI / 2.);
        transform.rotate_local_y(PI * -0.7);
        Some(transform)
    }

    pub fn challenger_position(
        &self,
        reference: &MaterializedSceneReference,
        id: usize,
    ) -> Option<Transform> {
        info!("Getting Challenger {id} Reference {reference:?}");
        let gltf = self.0.scenes.get(&reference.gltf)?;
        info!("Got GLTF Handle");
        let gltf = self.2.get(gltf)?;

        info!("got gltf");
        let challenger_node = gltf.named_nodes.get(&format!("ch{id}"))?;
        info!("found challenger node");
        let challenger_node = self.3.get(challenger_node)?;
        info!("got challenger transform");
        let mut transform = challenger_node.transform;
        transform.rotate_local_x(-1. * PI / 2.);
        transform.rotate_local_y(PI * -1.3);
        Some(transform)
    }
}
