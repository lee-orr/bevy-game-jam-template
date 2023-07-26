#![allow(dead_code)]
use bevy::prelude::{AmbientLight, Color};

pub const KNIGHTS_MAIN: Color = Color::rgb(0.47, 0.47, 0.47);
pub const KNIGHTS_SECONDARY: Color = Color::rgb(0.98, 0.75, 0.31);

pub const DRUIDS_MAIN: Color = Color::rgb(0.27, 0.5, 0.39);
pub const DRUIDS_SECONDARY: Color = Color::rgb(0.27, 0.15, 0.66);

pub const SCREEN_BACKGROUND_COLOR: Color = Color::rgb(0.54, 0.31, 0.12);

pub const BORDER_COLOR: Color = Color::rgb(0.19, 0.25, 0.35);
pub const PRIMARY_BACKGROUND_COLOR: Color = Color::rgb(0.73, 0.26, 0.14);
pub const PRIMARY_COLOR: Color = Color::rgb(0.24, 0.63, 0.89);

pub const PRIMARY_COLOR_PRIORITIZED: Color = Color::rgb(0.06, 0.56, 0.89);
pub const PRIMARY_COLOR_FOCUSED: Color = PRIMARY_COLOR_PRIORITIZED;
pub const PRIMARY_COLOR_ACTIVE: Color = PRIMARY_COLOR_PRIORITIZED;
pub const PRIMARY_COLOR_BLOCKED: Color = Color::rgb(0.48, 0.64, 0.74);
pub const PRIMARY_BUTTON_TEXT: Color = Color::rgb(0.07, 0.15, 0.12);

pub const OVERLAY_COLOR: Color = Color::rgba(0., 0., 0., 0.9);

pub const CRITICAL_COLOR: Color = Color::rgb(0.9, 0.64, 0.26);
pub const SUCCESS_COLOR: Color = Color::rgb(0.04, 0.91, 0.66);
pub const FAIL_COLOR: Color = Color::rgb(0.44, 0.29, 0.31);
pub const CRITICAL_FAIL_COLOR: Color = Color::rgb(0.98, 0.04, 0.3);

pub const CARD_COLOR: Color = Color::rgb(0.35, 0.42, 0.4);
pub const CARD_COLOR_PRIORITIZED: Color = PRIMARY_BACKGROUND_COLOR;
pub const CARD_COLOR_FOCUSED: Color = Color::rgb(0.23, 0.47, 0.4);
pub const CARD_COLOR_ACTIVE: Color = Color::rgb(0.18, 0.38, 0.32);
pub const CARD_COLOR_BLOCKED: Color = Color::rgb(0.4, 0.3, 0.4);

pub const VISUALIZER_BACKGROUND: Color = Color::rgba(0.9, 0.9, 0.9, 0.3);
pub const POWER_TOOLBAR_COLOR: Color = Color::rgba(0.83, 0.95, 0.83, 0.3);

pub const ENCOUNTER_COLOR_PRIORITIZED: Color = PRIMARY_BACKGROUND_COLOR;
pub const ENCOUNTER_COLOR_FOCUSED: Color = Color::rgb(0.23, 0.47, 0.4);
pub const ENCOUNTER_COLOR_ACTIVE: Color = Color::rgb(0.18, 0.38, 0.32);
pub const ENCOUNTER_COLOR_BLOCKED: Color = Color::rgb(0.4, 0.3, 0.4);

pub const ENCOUNTER_DRUID_COLOR_PRIORITIZED: Color = Color::rgb(0.19, 0.1, 0.46);
pub const ENCOUNTER_DRUID_COLOR_FOCUSED: Color = ENCOUNTER_DRUID_COLOR_PRIORITIZED;
pub const ENCOUNTER_DRUID_COLOR_ACTIVE: Color = Color::rgb(0.37, 0.2, 0.91);
pub const ENCOUNTER_DRUID_COLOR_BLOCKED: Color = Color::rgb(0.4, 0.3, 0.4);

pub const ENCOUNTER_KNIGHT_COLOR_PRIORITIZED: Color = Color::rgb(0.9, 0.64, 0.16);
pub const ENCOUNTER_KNIGHT_COLOR_FOCUSED: Color = ENCOUNTER_KNIGHT_COLOR_PRIORITIZED;
pub const ENCOUNTER_KNIGHT_COLOR_ACTIVE: Color = Color::rgb(0.74, 0.56, 0.22);
pub const ENCOUNTER_KNIGHT_COLOR_BLOCKED: Color = Color::rgb(0.4, 0.3, 0.4);

pub const DEFAULT_AMBIENT: AmbientLight = AmbientLight {
    color: Color::rgb(1., 1., 1.),
    brightness: 0.26,
};

pub const DEFAULT_CLEAR: Color = Color::rgb(0.75, 0.75, 0.75);
