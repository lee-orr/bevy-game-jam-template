use bevy::{prelude::*, reflect::Reflect};
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use bevy_ui_dsl::{node, UiChildBuilder};

use crate::{
    assets::MainGameAssets,
    ui::{colors, spawn_icon, DisplayBundle},
};

use super::dice_pools::{DicePool, DicePoolType, DiceType};

#[derive(Component, InspectorOptions, Reflect, Clone, Copy)]
#[reflect(InspectorOptions)]
pub enum Power {
    SplitDice,
    // CombineDice,
    AddDice(DiceType),
    Advantage,
    StaticBonus(u8),
    // DefaceDice(DiceType),
    // ExplodeDice,
    // PreRoll,
    // ReRoll,
}

#[derive(InspectorOptions, Reflect, Default)]
#[reflect(InspectorOptions)]
pub enum PowerTargetingType {
    #[default]
    Single,
    Action,
}

impl Power {
    pub fn targets(&self) -> PowerTargetingType {
        match self {
            Power::AddDice(_) => PowerTargetingType::Action,
            Power::StaticBonus(_) => PowerTargetingType::Action,
            _ => PowerTargetingType::Single,
        }
    }

    pub fn valid_targets(&self, input: &[&DicePool]) -> bool {
        match self {
            Power::SplitDice => {
                input.len() == 1
                    && if let Some(first) = input.first() {
                        first.dice != DiceType::D2 && first.dice != DiceType::D3
                    } else {
                        false
                    }
            }
            Power::AddDice(_) => input.is_empty(),
            Power::Advantage => {
                input.len() == 1
                    && if let Some(DicePool { dice, pool }) = input.first() {
                        if *pool == DicePoolType::Advantage {
                            return false;
                        }
                        !matches!(dice, DiceType::Static { value: _ })
                    } else {
                        true
                    }
            }
            Power::StaticBonus(_) => input.is_empty(),
        }
    }

    pub fn apply(&self, input: &[&DicePool]) -> Vec<DicePool> {
        match self {
            Power::SplitDice => input
                .iter()
                .flat_map(|v| {
                    let (count, dice) = match v.dice {
                        DiceType::D4 => (2usize, DiceType::D2),
                        DiceType::D6 => (2, DiceType::D3),
                        DiceType::D8 => (2, DiceType::D4),
                        DiceType::D12 => (2, DiceType::D6),
                        _ => (1, v.dice),
                    };
                    let pool = v.pool;

                    (0..count)
                        .map(|_| DicePool { dice, pool })
                        .collect::<Vec<_>>()
                })
                .collect(),
            Power::AddDice(d) => vec![DicePool::new(*d)],
            Power::Advantage => input.iter().map(|v| (**v).advantage()).collect(),
            Power::StaticBonus(v) => vec![DicePool::bonus(*v)],
        }
    }
}

impl Default for Power {
    fn default() -> Self {
        Self::StaticBonus(1)
    }
}

impl DisplayBundle for Power {
    fn display_bundle(&self, assets: &MainGameAssets, icon_size: f32, parent: &mut UiChildBuilder) {
        node(
            |b: &mut NodeBundle| {
                b.style.width = Val::Px(icon_size);
                b.style.height = Val::Px(icon_size);
                b.style.align_items = AlignItems::Center;
                b.style.justify_content = JustifyContent::Center;
            },
            parent,
            |parent| {
                match self {
                    Power::SplitDice => {
                        parent.spawn(spawn_icon(8, assets, icon_size));
                    }
                    Power::AddDice(dice) => {
                        parent
                            .spawn(NodeBundle {
                                ..Default::default()
                            })
                            .with_children(|p| {
                                p.spawn(TextBundle::from_section(
                                    "+".to_string(),
                                    TextStyle {
                                        font: assets.knights_font.clone(),
                                        font_size: 40.,
                                        color: colors::PRIMARY_BUTTON_TEXT,
                                    },
                                ));
                                dice.display_bundle(assets, icon_size, p);
                            });
                    }
                    Power::Advantage => {
                        parent.spawn(spawn_icon(9, assets, icon_size));
                    }
                    Power::StaticBonus(v) => {
                        parent.spawn(
                            TextBundle::from_section(
                                format!("+{v}"),
                                TextStyle {
                                    font: assets.druids_font.clone(),
                                    font_size: 30.,
                                    color: colors::PRIMARY_BUTTON_TEXT,
                                },
                            )
                            .with_style(Style {
                                padding: UiRect::all(Val::Px(5.)),
                                ..default()
                            }),
                        );
                    }
                };
            },
        );
    }
}
