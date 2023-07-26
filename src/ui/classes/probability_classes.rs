use crate::ui::intermediary_node_bundles::IntermediaryNodeBundleHandler;

use super::super::colors;
use bevy::prelude::*;
use bevy_ui_navigation::prelude::FocusState;

pub fn c_probability_setup_root(b: &mut NodeBundle) {
    b.style.width = Val::Percent(100.);
    b.style.height = Val::Percent(100.);
    b.style.display = Display::Flex;
    b.style.flex_direction = FlexDirection::Column;
    b.style.justify_content = JustifyContent::FlexEnd;
    b.style.align_items = AlignItems::Center;
    b.style.padding = UiRect::all(Val::Px(30.));
    b.style.row_gap = Val::Px(5.);
    b.style.column_gap = Val::Px(5.);
}

pub fn probability_grid(b: &mut NodeBundle) {
    b.style.display = Display::Flex;
    b.style.flex_wrap = FlexWrap::Wrap;
    b.style.row_gap = Val::Px(5.);
    b.style.column_gap = Val::Px(5.);
    b.style.flex_direction = FlexDirection::Row;
    b.style.justify_content = JustifyContent::Center;
}

pub fn probability_card(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.style().border = UiRect::all(Val::Px(2.));
    b.background_color().0 = colors::CARD_COLOR;

    b.style().display = Display::Grid;
    b.style().grid_template_rows = vec![GridTrack::fr(1.), GridTrack::fr(1.), GridTrack::fr(3.)];
    b.style().height = Val::VMin(20.);
    b.style().width = Val::VMin(30.);
}

pub fn player_card(b: &mut dyn IntermediaryNodeBundleHandler) {
    if let Some(b) = b.border_color() {
        b.0 = colors::SUCCESS_COLOR;
    }
}

pub fn challenger_card(b: &mut dyn IntermediaryNodeBundleHandler) {
    if let Some(b) = b.border_color() {
        b.0 = colors::FAIL_COLOR;
    }
}

pub fn probability_card_title(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.style().padding = UiRect::all(Val::Px(5.));
    b.style().justify_content = JustifyContent::Center;
    b.style().align_items = AlignItems::Center;
    b.background_color().0 = colors::OVERLAY_COLOR;
}

pub fn probability_card_title_text(_: &AssetServer, t: &mut TextStyle) {
    t.font_size = 20.;
    t.color = colors::PRIMARY_COLOR;
}

pub fn probability_card_dice_pool_container(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.style().padding = UiRect::all(Val::Px(5.));
    b.style().justify_content = JustifyContent::Center;
    b.style().align_items = AlignItems::Center;
}

pub fn probability_card_visualizer(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.style().justify_content = JustifyContent::Center;
    b.style().align_items = AlignItems::FlexEnd;
    b.style().flex_direction = FlexDirection::Row;
    b.style().width = Val::Percent(100.);
    b.background_color().0 = colors::VISUALIZER_BACKGROUND;
}

pub fn probability_power_container(b: &mut NodeBundle) {
    b.background_color.0 = colors::POWER_TOOLBAR_COLOR;
    b.style.padding = UiRect::all(Val::Px(5.));
    b.style.row_gap = Val::Px(5.);
    b.style.column_gap = Val::Px(5.);
}

pub fn powers_container(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.style().row_gap = Val::Px(5.);
    b.style().column_gap = Val::Px(5.);
}

pub fn power_card_container(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.background_color().0 = colors::PRIMARY_COLOR;
    b.style().padding = UiRect::all(Val::Px(5.));
}

pub fn power_card_prioritized(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.background_color().0 = colors::PRIMARY_COLOR_PRIORITIZED;
}

pub fn power_card_focused(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.background_color().0 = colors::PRIMARY_COLOR_PRIORITIZED;
}

pub fn power_card_active(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.background_color().0 = colors::PRIMARY_COLOR_ACTIVE;
}

pub fn power_card_blocked(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.background_color().0 = colors::PRIMARY_COLOR_BLOCKED;
}

pub fn apply_power_card_state(state: FocusState) -> NodeBundle {
    let mut bundle = NodeBundle::default();
    power_card_container(&mut bundle);
    match state {
        FocusState::Prioritized => power_card_prioritized(&mut bundle),
        FocusState::Focused => power_card_focused(&mut bundle),
        FocusState::Active => power_card_active(&mut bundle),
        FocusState::Blocked => power_card_blocked(&mut bundle),
        FocusState::Inert => {}
    };
    bundle
}

pub fn apply_action_state(state: FocusState) -> NodeBundle {
    let mut bundle = NodeBundle::default();
    probability_card(&mut bundle);
    match state {
        FocusState::Prioritized => power_card_prioritized(&mut bundle),
        FocusState::Focused => power_card_focused(&mut bundle),
        FocusState::Active => power_card_active(&mut bundle),
        FocusState::Blocked => power_card_blocked(&mut bundle),
        FocusState::Inert => {}
    };
    bundle
}

pub fn apply_action_state_ch(state: FocusState) -> NodeBundle {
    let mut result = apply_action_state(state);
    challenger_card(&mut result);
    result
}

pub fn apply_action_state_pl(state: FocusState) -> NodeBundle {
    let mut result = apply_action_state(state);
    player_card(&mut result);
    result
}
