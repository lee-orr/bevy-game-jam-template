use bevy::prelude::*;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

#[derive(Clone, Eq, PartialEq, Copy, Debug, Hash, Default, States, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub enum EncounterState {
    #[default]
    None,
    Loading,
    Introduction,
    ActionChoice,
    ProbabilitySetup,
    OutcomeResolution,
    CheckEncounterResolution,
    EncounterResolved,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct ClearUnusuedActions;

#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PublishAvailableActions;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct FlushAvailableActions;
