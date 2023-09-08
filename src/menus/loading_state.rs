use std::ops::Mul;

use bevy::prelude::*;
use bevy_vector_shapes::{prelude::ShapePainter, shapes::DiscPainter};

use crate::{app_state::AppState, ui::colors};
use dexterous_developer::{
    dexterous_developer_setup, ReloadableApp, ReloadableAppContents, ReloadableElementsSetup,
};

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.setup_reloadable_elements::<reloadable>();
    }
}

#[dexterous_developer_setup(loading)]
fn reloadable(app: &mut ReloadableAppContents) {
    app.add_systems(Update, draw_loading.run_if(in_state(AppState::LoadingMenu)));
}

const LOADING_ANIMATION_SPEED: f32 = 5.;
const LOADING_ANIMATION_SIZE: f32 = 50.0;
const LOADING_ANIMATION_VERTICAL: f32 = 20.0;
const CIRCLE_SIZE: f32 = 15.;
const CIRCLE_DELAY: f32 = 2.;
const NUM_CIRCLES: u8 = 4;

fn draw_loading(mut painter: ShapePainter, time: Res<Time>) {
    painter.set_2d();

    let left_edge = (NUM_CIRCLES as f32 * LOADING_ANIMATION_SIZE) / -2.;

    for (circle, color) in [
        colors::PRIMARY_COLOR,
        colors::PRIMARY_COLOR,
        colors::PRIMARY_COLOR,
    ]
    .iter()
    .enumerate()
    {
        let time_offset =
            (time.elapsed_seconds() - CIRCLE_DELAY * (circle as f32)).mul(LOADING_ANIMATION_SPEED);
        let offset_y = time_offset.cos();
        let location = Vec3::new(
            left_edge + (circle as f32) * LOADING_ANIMATION_SIZE,
            offset_y * LOADING_ANIMATION_VERTICAL,
            0.,
        );

        painter.set_translation(location);
        painter.color = *color;
        painter.circle(CIRCLE_SIZE);
    }
}
