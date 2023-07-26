use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy_vector_shapes::{
    prelude::ShapePainter,
    shapes::{Cap, LinePainter},
};
use serde::Deserialize;

use crate::{in_game::game_state::GameState, ui::colors};

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CurrentHealth>()
            .register_type::<MaxHealth>()
            .add_systems(
                Update,
                (draw_health_bars).run_if(in_state(GameState::Encounter)),
            );
    }
}

#[derive(Component, Reflect, InspectorOptions, Default)]
pub struct CurrentHealth(pub usize);

#[derive(Component, Reflect, InspectorOptions, Default, Clone, Copy, Debug, Deserialize)]
pub struct MaxHealth(pub usize);

const HEALTH_BAR_END: Vec3 = Vec3::new(50., 0., 0.);
const HEALTH_BAR_OFFSET: Vec3 = Vec3::new(-25., 0., 0.);
const HEALTH_BAR_WIDTH: f32 = 15.;

fn draw_health_bars(
    mut painter: ShapePainter,
    creatures: Query<(&GlobalTransform, &CurrentHealth, &MaxHealth)>,
    camera: Query<(&GlobalTransform, &Camera), With<Camera3d>>,
    camera_2d: Query<(&GlobalTransform, &Camera), With<Camera2d>>,
) {
    let Ok((camera_transform, camera)) = camera.get_single() else {
        return;
    };
    let Ok((camera_2d_transform, camera_2d)) = camera_2d.get_single() else {
        return;
    };

    for (transform, health, max) in creatures.iter() {
        let Some(position) = camera.world_to_ndc(camera_transform, transform.translation()) else {
            continue;
        };
        let Some(position) = camera_2d.ndc_to_world(camera_2d_transform, position) else {
            continue;
        };

        painter.set_translation(position + HEALTH_BAR_OFFSET);

        painter.thickness = HEALTH_BAR_WIDTH;
        painter.color = colors::OVERLAY_COLOR;
        painter.cap = Cap::Round;
        painter.line(Vec3::ZERO, HEALTH_BAR_END);

        let health_ratio = (health.0 as f32) / (max.0 as f32);

        let health_color = match health_ratio {
            x if x > 0.8 => colors::CRITICAL_COLOR,
            x if x > 0.4 => colors::SUCCESS_COLOR,
            _ => colors::CRITICAL_FAIL_COLOR,
        };

        painter.thickness = HEALTH_BAR_WIDTH;
        painter.color = health_color;
        painter.cap = Cap::Round;
        painter.line(Vec3::ZERO, HEALTH_BAR_END * health_ratio);
    }
}
