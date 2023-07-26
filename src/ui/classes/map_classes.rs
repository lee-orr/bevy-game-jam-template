use super::super::colors;
use bevy::prelude::*;

pub fn map_powers_root(b: &mut NodeBundle) {
    b.style.width = Val::Percent(100.);
    b.style.height = Val::Percent(100.);
    b.style.flex_direction = FlexDirection::ColumnReverse;
    b.style.justify_content = JustifyContent::FlexStart;
    b.style.align_items = AlignItems::FlexStart;
}

pub fn map_powers_container(b: &mut NodeBundle) {
    b.style.padding = UiRect::all(Val::Px(5.));
    b.style.row_gap = Val::Px(5.);
    b.style.column_gap = Val::Px(5.);
    b.background_color.0 = colors::VISUALIZER_BACKGROUND;
    b.style.justify_content = JustifyContent::Center;
    b.style.align_items = AlignItems::Center;
}

pub fn map_power_card(b: &mut NodeBundle) {
    b.style.flex_grow = 0.;
    b.style.flex_shrink = 0.;
}

pub fn map_powers_overlay(b: &mut NodeBundle) {
    b.style.position_type = PositionType::Absolute;
    b.style.top = Val::Px(0.);
    b.style.bottom = Val::Px(0.);
    b.style.left = Val::Px(0.);
    b.style.right = Val::Px(0.);
    b.background_color.0 = colors::VISUALIZER_BACKGROUND;
}
