use crate::ui::intermediary_node_bundles::IntermediaryNodeBundleHandler;

use super::super::colors;
use bevy::prelude::*;

pub fn mission_root(b: &mut NodeBundle) {
    b.style.width = Val::Percent(100.);
    b.style.height = Val::Percent(100.);
    b.style.flex_direction = FlexDirection::Column;
    b.style.justify_content = JustifyContent::Center;
    b.style.align_items = AlignItems::Center;
    b.background_color = BackgroundColor(colors::SCREEN_BACKGROUND_COLOR);
}

pub fn mission_container(b: &mut NodeBundle) {
    b.style.padding = UiRect::all(Val::Px(10.));
    b.style.row_gap = Val::Px(15.);
    b.style.column_gap = Val::Px(15.);
    b.background_color.0 = colors::OVERLAY_COLOR;
    b.style.flex_direction = FlexDirection::Column;
    b.style.justify_content = JustifyContent::Center;
    b.style.align_items = AlignItems::Center;
}

pub fn mission_encounter_title(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.style().padding = UiRect::all(Val::Px(5.));
}
