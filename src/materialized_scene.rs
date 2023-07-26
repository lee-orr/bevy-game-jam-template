use bevy::{ecs::query::Has, prelude::*};
use bevy_inspector_egui::InspectorOptions;
use serde::Deserialize;

use crate::toon_material::ToonMaterial;

pub struct SceneSpawnerPlugin;

impl Plugin for SceneSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MaterializedScene>()
            .add_systems(PreUpdate, spawn_scene)
            .add_systems(PostUpdate, set_material);
    }
}

#[derive(Component, Reflect, InspectorOptions, Default, Clone)]
pub struct MaterializedScene {
    pub scene: Handle<Scene>,
    pub material: Handle<ToonMaterial>,
}

#[derive(Component, Reflect, Deserialize, Default, Clone, Debug)]
pub struct MaterializedSceneReference {
    pub gltf: String,
    pub scene: String,
    pub pallet: Vec<String>,
}

#[derive(Bundle, Default)]
pub struct MaterializedSceneBundle {
    pub spawner: MaterializedScene,
    pub transform: TransformBundle,
    pub visibility: VisibilityBundle,
}

impl Clone for MaterializedSceneBundle {
    fn clone(&self) -> Self {
        Self {
            spawner: self.spawner.clone(),
            transform: self.transform,
            visibility: VisibilityBundle {
                visibility: self.visibility.visibility,
                computed: Default::default(),
            },
        }
    }
}

fn spawn_scene(
    mut commands: Commands,
    spawners: Query<(Entity, &MaterializedScene), Without<Handle<Scene>>>,
) {
    for (entity, scene) in spawners.iter() {
        let mut cmd = commands.entity(entity);
        cmd.insert(scene.scene.clone());
    }
}

type MaterialQuery<'w, 's, 'a> =
    Query<'w, 's, (Entity, Option<&'a Children>, Has<Handle<StandardMaterial>>), With<Parent>>;

fn set_material(
    mut commands: Commands,
    spawners: Query<(Entity, &MaterializedScene, &Children), With<Handle<Scene>>>,
    query: MaterialQuery,
) {
    for (entity, scene, children) in spawners.iter() {
        commands
            .entity(entity)
            .remove::<MaterializedScene>()
            .insert(Visibility::Visible);
        set_material_internal(&mut commands, children, &scene.material, &query);
    }
}

fn set_material_internal(
    commands: &mut Commands,
    children: &Children,
    material: &Handle<ToonMaterial>,
    query: &MaterialQuery,
) {
    for (entity, children, has_material) in children.iter().filter_map(|e| query.get(*e).ok()) {
        if has_material {
            commands
                .entity(entity)
                .remove::<Handle<StandardMaterial>>()
                .insert(material.clone());
        }
        if let Some(children) = children {
            set_material_internal(commands, children, material, query);
        }
    }
}
