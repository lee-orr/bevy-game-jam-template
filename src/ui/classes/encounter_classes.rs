use bevy::prelude::*;
use bevy_ui_navigation::prelude::FocusState;

use crate::ui::{colors, intermediary_node_bundles::IntermediaryNodeBundleHandler};

pub fn encounter_listing(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.style().padding = UiRect::all(Val::Px(30.));
    b.background_color().0 = colors::OVERLAY_COLOR;
    b.style().bottom = Val::Px(20.);
}

pub fn encounter_prioritized(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.background_color().0 = colors::PRIMARY_COLOR_PRIORITIZED;
}

pub fn encounter_focused(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.background_color().0 = colors::PRIMARY_COLOR_FOCUSED;
}

pub fn encounter_active(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.background_color().0 = colors::PRIMARY_COLOR_ACTIVE;
}

pub fn encounter_blocked(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.background_color().0 = colors::PRIMARY_COLOR_BLOCKED;
}

pub fn apply_encounter_state(state: FocusState) -> NodeBundle {
    let mut bundle = NodeBundle::default();
    encounter_listing(&mut bundle);
    match state {
        FocusState::Prioritized => encounter_prioritized(&mut bundle),
        FocusState::Focused => encounter_focused(&mut bundle),
        FocusState::Active => encounter_active(&mut bundle),
        FocusState::Blocked => encounter_blocked(&mut bundle),
        FocusState::Inert => {}
    };
    bundle
}

pub fn encounter_druid_listing(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.style().padding = UiRect::all(Val::Px(30.));
    b.background_color().0 = colors::DRUIDS_SECONDARY;
    b.style().bottom = Val::Px(20.);
}

pub fn encounter_druid_prioritized(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.background_color().0 = colors::ENCOUNTER_DRUID_COLOR_PRIORITIZED;
}

pub fn encounter_druid_focused(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.background_color().0 = colors::ENCOUNTER_DRUID_COLOR_FOCUSED;
}

pub fn encounter_druid_active(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.background_color().0 = colors::ENCOUNTER_DRUID_COLOR_ACTIVE;
}

pub fn encounter_druid_blocked(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.background_color().0 = colors::ENCOUNTER_DRUID_COLOR_BLOCKED;
}

pub fn apply_encounter_druid_state(state: FocusState) -> NodeBundle {
    let mut bundle = NodeBundle::default();
    encounter_druid_listing(&mut bundle);
    match state {
        FocusState::Prioritized => encounter_druid_prioritized(&mut bundle),
        FocusState::Focused => encounter_druid_focused(&mut bundle),
        FocusState::Active => encounter_druid_active(&mut bundle),
        FocusState::Blocked => encounter_druid_blocked(&mut bundle),
        FocusState::Inert => {}
    };
    bundle
}

pub fn encounter_knight_listing(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.style().padding = UiRect::all(Val::Px(30.));
    b.background_color().0 = colors::KNIGHTS_SECONDARY;
    b.style().bottom = Val::Px(20.);
}

pub fn encounter_knight_prioritized(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.background_color().0 = colors::PRIMARY_COLOR_PRIORITIZED;
}

pub fn encounter_knight_focused(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.background_color().0 = colors::ENCOUNTER_KNIGHT_COLOR_PRIORITIZED;
}

pub fn encounter_knight_active(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.background_color().0 = colors::ENCOUNTER_KNIGHT_COLOR_ACTIVE;
}

pub fn encounter_knight_blocked(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.background_color().0 = colors::ENCOUNTER_KNIGHT_COLOR_BLOCKED;
}

pub fn apply_encounter_knight_state(state: FocusState) -> NodeBundle {
    let mut bundle = NodeBundle::default();
    encounter_knight_listing(&mut bundle);
    match state {
        FocusState::Prioritized => encounter_knight_prioritized(&mut bundle),
        FocusState::Focused => encounter_knight_focused(&mut bundle),
        FocusState::Active => encounter_knight_active(&mut bundle),
        FocusState::Blocked => encounter_knight_blocked(&mut bundle),
        FocusState::Inert => {}
    };
    bundle
}
