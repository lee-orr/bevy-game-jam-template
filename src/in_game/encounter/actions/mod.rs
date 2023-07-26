mod combat;
mod text;

use bevy::{prelude::*, reflect::Reflect};
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use serde::Deserialize;

use self::{combat::CombatActionPlugin, text::TextActionPlugin};

use super::dice_pools::InitialPools;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<ActionChoice>()
            .register_type::<ActionResult>()
            .register_type::<Resolution>()
            .register_type::<ActionType>()
            .add_plugins(TextActionPlugin)
            .add_plugins(CombatActionPlugin);
    }
}

#[derive(Component, InspectorOptions, Reflect, Deserialize, Clone, Debug)]
#[reflect(InspectorOptions)]
pub struct ActionChoice {
    pub title: String,
    pub content: String,
    pub fail: u8,
    pub success: u8,
    pub critical_success: u8,
    pub dice_pool: InitialPools,
}

impl Default for ActionChoice {
    fn default() -> Self {
        Self {
            title: "An Action".to_string(),
            content: Default::default(),
            fail: 2,
            success: 6,
            critical_success: 9,
            dice_pool: Default::default(),
        }
    }
}

#[derive(InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub enum ActionResult {
    CriticalFail,
    Fail,
    Success,
    CriticalSuccess,
}

impl ActionChoice {
    pub fn evaluate(&self, value: u8) -> (ActionResult, u8) {
        if value < self.fail {
            (ActionResult::CriticalFail, self.fail - value)
        } else if value < self.success {
            (ActionResult::Fail, self.success - value)
        } else if value < self.critical_success {
            (ActionResult::Success, value - self.success)
        } else {
            (ActionResult::CriticalSuccess, value - self.critical_success)
        }
    }
}

#[derive(Component, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub struct Resolution {
    pub roll: u8,
    pub result: ActionResult,
    pub gap: u8,
}

#[derive(Component)]
pub struct ChosenAction;

#[derive(Component, Default)]
pub struct ChallengerAction;

#[derive(Bundle, Default)]
pub struct PlayerActionBundle {
    pub action_choice: ActionChoice,
    pub action_type: ActionType,
    pub target: ActionTarget,
}

#[derive(Bundle, Default)]
pub struct ChallengerActionBundle {
    pub action_choice: ActionChoice,
    pub challenger_action: ChallengerAction,
    pub action_type: ActionType,
    pub target: ActionTarget,
}

#[derive(Component, InspectorOptions, Reflect, Clone, Debug, Default)]
#[reflect(InspectorOptions)]
pub struct ActionTarget(pub Option<Entity>);

#[derive(Component, InspectorOptions, Reflect, Deserialize, Default, Clone, Debug)]
#[reflect(InspectorOptions)]
pub enum ActionType {
    #[default]
    Text,
    Attack {
        base_damage: u8,
    },
}

#[derive(Reflect, InspectorOptions, Deserialize, Default, Clone, Debug)]
pub struct ActionDefinition {
    pub choice: ActionChoice,
    pub action_type: ActionType,
}
