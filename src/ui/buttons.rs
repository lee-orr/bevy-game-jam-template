use bevy::prelude::*;
use bevy_ui_dsl::*;
use bevy_ui_navigation::{
    components::FocusableButtonBundle,
    prelude::{FocusState, Focusable, Focused, NavRequest},
};

pub type FocusedButtonQuery<'w, 's, 'a> = Query<'w, 's, Entity, (With<Focused>, With<Button>)>;
pub type TypedFocusedButtonQuery<'w, 's, 'a, T> =
    Query<'w, 's, (Entity, &'a T), (With<Focused>, With<Button>)>;

type StyledButtonQuery<'w, 's, 'a> = Query<
    'w,
    's,
    (Entity, &'a Focusable, &'a StyledFocusButton),
    (Changed<Focusable>, With<Button>),
>;

/// Spawns a [`ButtonBundle`] with children.
pub fn focus_button(
    class: impl AssetClass<FocusableButtonBundle>,
    state_styler: impl 'static + Sync + Send + Fn(FocusState) -> NodeBundle,
    parent: &mut UiChildBuilder,
    children: impl FnOnce(&mut UiChildBuilder),
) -> Entity {
    let mut bundle = FocusableButtonBundle::default();
    class.apply(parent.assets(), &mut bundle);
    parent
        .spawn((bundle, StyledFocusButton(Box::new(state_styler))))
        .with_children(children)
        .id()
}

/// Spawns a [`ButtonBundle`] with a single [`TextBundle`] as its child.
pub fn focus_text_button(
    txt: impl Into<String>,
    class: impl AssetClass<FocusableButtonBundle>,
    state_styler: impl 'static + Sync + Send + Fn(FocusState) -> NodeBundle,
    text_style: impl AssetClass<TextStyle>,
    parent: &mut UiChildBuilder,
) -> Entity {
    focus_button(class, state_styler, parent, |p| {
        text(txt, (), text_style, p);
    })
}

#[derive(Component)]
pub struct StyledFocusButton(Box<dyn 'static + Sync + Send + Fn(FocusState) -> NodeBundle>);

pub fn apply_button_styles(mut commands: Commands, query: StyledButtonQuery) {
    for (entity, focusable, style) in query.iter() {
        commands.entity(entity).insert(style.0(focusable.state()));
    }
}

pub fn focused_button_activated(
    mut events: EventReader<NavRequest>,
    focused: FocusedButtonQuery,
) -> Option<Entity> {
    for event in events.iter() {
        if let NavRequest::Action = event {
            let focused = focused.get_single().ok();
            return focused;
        }
    }
    None
}
