use bevy_ui_dsl::Class;

use bevy_ui_dsl::AssetClass;

use bevy::prelude::*;
use bevy_ui_navigation::components::FocusableButtonBundle;

pub trait IntermediaryNodeBundleHandler {
    fn style(&mut self) -> &mut Style;
    fn background_color(&mut self) -> &mut BackgroundColor;
    fn border_color(&mut self) -> Option<&mut BorderColor>;
    fn z_index(&mut self, z_index: ZIndex);
}

impl IntermediaryNodeBundleHandler for NodeBundle {
    fn style(&mut self) -> &mut Style {
        &mut self.style
    }

    fn background_color(&mut self) -> &mut BackgroundColor {
        &mut self.background_color
    }

    fn border_color(&mut self) -> Option<&mut BorderColor> {
        Some(&mut self.border_color)
    }

    fn z_index(&mut self, z_index: ZIndex) {
        self.z_index = z_index;
    }
}

impl IntermediaryNodeBundleHandler for TextBundle {
    fn style(&mut self) -> &mut Style {
        &mut self.style
    }

    fn background_color(&mut self) -> &mut BackgroundColor {
        &mut self.background_color
    }

    fn border_color(&mut self) -> Option<&mut BorderColor> {
        None
    }

    fn z_index(&mut self, z_index: ZIndex) {
        self.z_index = z_index;
    }
}

impl IntermediaryNodeBundleHandler for ButtonBundle {
    fn style(&mut self) -> &mut Style {
        &mut self.style
    }

    fn background_color(&mut self) -> &mut BackgroundColor {
        &mut self.background_color
    }

    fn border_color(&mut self) -> Option<&mut BorderColor> {
        Some(&mut self.border_color)
    }

    fn z_index(&mut self, z_index: ZIndex) {
        self.z_index = z_index;
    }
}

impl IntermediaryNodeBundleHandler for FocusableButtonBundle {
    fn style(&mut self) -> &mut Style {
        &mut self.button_bundle.style
    }

    fn background_color(&mut self) -> &mut BackgroundColor {
        &mut self.button_bundle.background_color
    }

    fn border_color(&mut self) -> Option<&mut BorderColor> {
        Some(&mut self.button_bundle.border_color)
    }

    fn z_index(&mut self, z_index: ZIndex) {
        self.button_bundle.z_index = z_index;
    }
}

pub(crate) type Inner = Box<dyn FnOnce(&mut dyn IntermediaryNodeBundleHandler)>;

pub struct IntermediaryNodeBundle(Inner);

impl AssetClass<TextBundle> for IntermediaryNodeBundle {
    fn apply(self, _assets: &AssetServer, b: &mut TextBundle) {
        self.0(b)
    }
}

impl AssetClass<ButtonBundle> for IntermediaryNodeBundle {
    fn apply(self, _assets: &AssetServer, b: &mut ButtonBundle) {
        self.0(b)
    }
}

impl AssetClass<FocusableButtonBundle> for IntermediaryNodeBundle {
    fn apply(self, _assets: &AssetServer, b: &mut FocusableButtonBundle) {
        self.0(b)
    }
}

impl Class<NodeBundle> for IntermediaryNodeBundle {
    fn apply(self, b: &mut NodeBundle) {
        self.0(b)
    }
}

impl<F: FnOnce(&mut dyn IntermediaryNodeBundleHandler) + 'static> From<F>
    for IntermediaryNodeBundle
{
    fn from(value: F) -> Self {
        IntermediaryNodeBundle(Box::new(value))
    }
}

pub trait IntoIntermediaryNodeBundle {
    fn nb(self) -> IntermediaryNodeBundle;
}

impl<F: FnOnce(&mut dyn IntermediaryNodeBundleHandler) + 'static> IntoIntermediaryNodeBundle for F {
    fn nb(self) -> IntermediaryNodeBundle {
        self.into()
    }
}
