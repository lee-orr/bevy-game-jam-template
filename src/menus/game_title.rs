use bevy::prelude::*;
use bevy_ui_dsl::*;

use crate::ui::{
    classes::{main_text, primary_box_main, span},
    intermediary_node_bundles::IntoIntermediaryNodeBundle,
};

pub fn game_title(p: &mut UiChildBuilder<'_, '_, '_, '_>) -> Entity {
    node((span.nb(), primary_box_main.nb()), p, |p| {
        text("My Game", (), main_text, p);
    })
}
