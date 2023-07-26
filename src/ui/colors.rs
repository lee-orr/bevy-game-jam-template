#![allow(dead_code)]
use bevy::prelude::{AmbientLight, Color};

pub const OVERLAY_COLOR: Color = Color::rgba(0., 0., 0., 0.9);
pub const BORDER_COLOR: Color = Color::rgb(0.19, 0.25, 0.35);

pub const SCREEN_BACKGROUND_COLOR: Color = Color::rgb(0.46, 0.76, 0.96);

pub const PRIMARY_BACKGROUND_COLOR: Color = Color::rgb(0.11, 0.36, 0.61);

pub const PRIMARY_COLOR: Color = Color::rgb(0.96, 0.82, 0.38);
pub const PRIMARY_COLOR_PRIORITIZED: Color = Color::rgb(0.98, 0.93, 0.67);
pub const PRIMARY_COLOR_FOCUSED: Color = PRIMARY_COLOR_PRIORITIZED;
pub const PRIMARY_COLOR_ACTIVE: Color = PRIMARY_COLOR_PRIORITIZED;
pub const PRIMARY_COLOR_BLOCKED: Color = Color::rgb(0.48, 0.64, 0.74);

pub const DEFAULT_AMBIENT: AmbientLight = AmbientLight {
    color: Color::rgb(1., 1., 1.),
    brightness: 0.26,
};

pub const DEFAULT_CLEAR: Color = Color::rgb(0.75, 0.75, 0.75);
