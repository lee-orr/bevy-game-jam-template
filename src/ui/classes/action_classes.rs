use crate::ui::{colors::*, intermediary_node_bundles::IntermediaryNodeBundleHandler};

use super::base_classes::{self};
use bevy::prelude::*;

use bevy_ui_navigation::prelude::FocusState;

use super::super::colors;

pub fn c_action_choice_root(b: &mut NodeBundle) {
    b.style.width = Val::Percent(100.);
    b.style.height = Val::Percent(100.);
    b.style.display = Display::Flex;
    b.style.flex_direction = FlexDirection::Row;
    b.style.justify_content = JustifyContent::Center;
    b.style.align_items = AlignItems::End;
    b.style.padding = UiRect::all(Val::Px(30.));
    b.style.column_gap = Val::Px(5.);
}

pub fn card(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.style().border = UiRect::all(Val::Px(2.));
    if let Some(b) = b.border_color() {
        b.0 = colors::BORDER_COLOR;
    }
    b.background_color().0 = colors::CARD_COLOR;

    b.style().display = Display::Grid;
    b.style().grid_template_rows = vec![
        GridTrack::fr(1.5),
        GridTrack::max_content(),
        GridTrack::fr(1.),
    ];
}

pub fn card_prioritized(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.background_color().0 = colors::CARD_COLOR_PRIORITIZED;
}

pub fn card_focused(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.background_color().0 = colors::CARD_COLOR_FOCUSED;
}

pub fn card_active(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.background_color().0 = colors::CARD_COLOR_ACTIVE;
}

pub fn card_blocked(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.background_color().0 = colors::CARD_COLOR_BLOCKED;
}

pub fn apply_card_state(state: FocusState) -> NodeBundle {
    let mut bundle = NodeBundle::default();
    card(&mut bundle);
    base_classes::primary_box_item(&mut bundle);
    match state {
        FocusState::Prioritized => card_prioritized(&mut bundle),
        FocusState::Focused => card_focused(&mut bundle),
        FocusState::Active => card_active(&mut bundle),
        FocusState::Blocked => card_blocked(&mut bundle),
        FocusState::Inert => {}
    };
    bundle
}

pub fn card_title(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.style().padding = UiRect::all(Val::Px(5.));
    b.style().grid_row = GridPlacement::start(1).set_span(1);
    b.style().flex_direction = FlexDirection::Row;
    b.style().align_items = AlignItems::Center;
    b.style().justify_content = JustifyContent::Center;
    b.background_color().0 = colors::OVERLAY_COLOR;
}

pub fn card_title_text(_: &AssetServer, t: &mut TextStyle) {
    t.font_size = 40.;
    t.color = PRIMARY_COLOR;
}

pub fn card_footer(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.style().padding = UiRect::all(Val::Px(5.));
    b.style().grid_row = GridPlacement::start(3).set_span(1);
    b.style().flex_direction = FlexDirection::Row;
    b.style().align_items = AlignItems::Center;
    b.style().justify_content = JustifyContent::SpaceBetween;
    b.background_color().0 = colors::OVERLAY_COLOR;
}

pub fn card_dice(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.style().flex_shrink = 10.;
    b.style().flex_grow = 0.;
    b.style().justify_content = JustifyContent::Center;
    b.style().align_items = AlignItems::Center;
}

pub fn card_fail(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.style().flex_direction = FlexDirection::Row;
    b.style().align_items = AlignItems::FlexEnd;
}

pub fn card_content(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.style().padding = UiRect::all(Val::Px(5.));
    b.style().grid_row = GridPlacement::start(2).set_span(1);
}

pub fn card_success(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.style().flex_direction = FlexDirection::RowReverse;
    b.style().align_items = AlignItems::FlexEnd;
    b.style().column_gap = Val::Px(5.);
}

pub fn card_fail_text(assets: &AssetServer, t: &mut TextStyle) {
    t.font_size = 20.;
    t.color = CRITICAL_FAIL_COLOR;
    t.font = assets.load("fonts/AMERSN__.ttf");
}

pub fn card_success_text(assets: &AssetServer, t: &mut TextStyle) {
    t.font_size = 20.;
    t.color = SUCCESS_COLOR;
    t.font = assets.load("fonts/AMERSN__.ttf");
}

pub fn card_critical(assets: &AssetServer, t: &mut TextStyle) {
    t.font_size = 20.;
    t.color = CRITICAL_COLOR;
    t.font = assets.load("fonts/AMERSN__.ttf");
}
