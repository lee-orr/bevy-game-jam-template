use bevy::{prelude::*, reflect::Reflect};
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use bevy_turborand::TurboRand;
use bevy_ui_dsl::{node, UiChildBuilder};
use serde::Deserialize;

use crate::{
    assets::MainGameAssets,
    ui::{
        classes::dice_pool_modifier, colors, intermediary_node_bundles::IntoIntermediaryNodeBundle,
        spawn_icon, DisplayBundle,
    },
};

use super::powers::Power;

pub trait Roll {
    fn roll(&self, rng: &mut impl TurboRand) -> u8;
}

#[derive(InspectorOptions, Reflect, Clone, Debug, Copy, PartialEq, Eq, Deserialize)]
#[reflect(InspectorOptions)]
pub enum DiceType {
    D2,
    D3,
    D4,
    D6,
    D8,
    D12,
    Static { value: u8 },
}

impl Default for DiceType {
    fn default() -> Self {
        Self::D12
    }
}

impl Roll for DiceType {
    fn roll(&self, rng: &mut impl TurboRand) -> u8 {
        match self {
            DiceType::Static { value } => *value,
            DiceType::D2 => rng.u8(1..3),
            DiceType::D3 => rng.u8(1..4),
            DiceType::D4 => rng.u8(1..5),
            DiceType::D6 => rng.u8(1..7),
            DiceType::D8 => rng.u8(1..9),
            DiceType::D12 => rng.u8(1..13),
        }
    }
}

impl DisplayBundle for DiceType {
    fn display_bundle(&self, assets: &MainGameAssets, icon_size: f32, parent: &mut UiChildBuilder) {
        let (is_atlas, position) = match self {
            DiceType::D2 => (true, 0),
            DiceType::D3 => (true, 1),
            DiceType::D4 => (true, 2),
            DiceType::D6 => (true, 3),
            DiceType::D8 => (true, 4),
            DiceType::D12 => (true, 5),
            DiceType::Static { value } => (false, *value),
        };
        if is_atlas {
            parent.spawn(spawn_icon(position as usize, assets, icon_size));
        } else {
            parent.spawn(TextBundle::from_section(
                format!("+{position}"),
                TextStyle {
                    font: assets.druids_font.clone(),
                    font_size: 20.,
                    color: colors::CRITICAL_COLOR,
                },
            ));
        }
    }
}

#[derive(InspectorOptions, Reflect, Default, PartialEq, Eq, Clone, Debug, Copy, Deserialize)]
#[reflect(InspectorOptions)]
pub enum DicePoolType {
    #[default]
    Single,
    Advantage,
}

#[derive(InspectorOptions, Reflect, Component, Clone, Debug, Copy, Default, Deserialize)]
#[reflect(InspectorOptions)]
pub struct DicePool {
    pub dice: DiceType,
    #[serde(default)]
    pub pool: DicePoolType,
}

impl DicePool {
    pub fn new(dice: DiceType) -> Self {
        Self {
            dice,
            ..Default::default()
        }
    }

    pub fn d2() -> Self {
        Self::new(DiceType::D2)
    }

    pub fn d3() -> Self {
        Self::new(DiceType::D4)
    }

    pub fn d4() -> Self {
        Self::new(DiceType::D4)
    }

    pub fn d6() -> Self {
        Self::new(DiceType::D6)
    }

    pub fn d8() -> Self {
        Self::new(DiceType::D8)
    }
    pub fn d12() -> Self {
        Self::new(DiceType::D12)
    }

    pub fn bonus(value: u8) -> Self {
        Self::new(DiceType::Static { value })
    }

    pub fn advantage(mut self) -> Self {
        self.pool = DicePoolType::Advantage;
        self
    }
}

impl Roll for DicePool {
    fn roll(&self, rng: &mut impl TurboRand) -> u8 {
        match self.pool {
            DicePoolType::Single => self.dice.roll(rng),
            DicePoolType::Advantage => self.dice.roll(rng).max(self.dice.roll(rng)),
        }
    }
}

impl DisplayBundle for DicePool {
    fn display_bundle(&self, assets: &MainGameAssets, icon_size: f32, parent: &mut UiChildBuilder) {
        if DicePoolType::Advantage == self.pool {
            node(dice_pool_modifier.nb(), parent, |p| {
                Power::Advantage.display_bundle(assets, icon_size * 0.75, p);
            });
        }
        self.dice.display_bundle(assets, icon_size, parent);
    }
}

#[derive(InspectorOptions, Reflect, Component, Deserialize, Clone, Debug)]
#[reflect(InspectorOptions)]
pub struct InitialPools(Vec<DicePool>);

impl Default for InitialPools {
    fn default() -> Self {
        Self(vec![DicePool::default()])
    }
}

impl InitialPools {
    pub fn new(pools: Vec<DicePool>) -> Self {
        Self(pools)
    }
    pub fn iter(&self) -> impl Iterator<Item = &DicePool> {
        self.0.iter()
    }
}

pub trait SimulateDice<const COUNT: usize> {
    fn simulate(&self, rng: &mut impl TurboRand) -> Vec<(u8, f32)>;
}

impl<T: Roll, const COUNT: usize> SimulateDice<COUNT> for T {
    fn simulate(&self, rng: &mut impl TurboRand) -> Vec<(u8, f32)> {
        let mut raw_results = Vec::with_capacity(COUNT);
        for _ in 0..COUNT {
            raw_results.push(self.roll(rng));
        }
        raw_results.sort();
        let (max, counts_by_value) = raw_results.iter().fold(
            (0usize, Vec::<(u8, usize)>::new()),
            |(max, mut val), next| {
                if let Some(last) = val.last_mut() {
                    if last.0 == *next {
                        let count = last.1 + 1;
                        last.1 = count;
                        return (max.max(count), val);
                    }
                }
                val.push((*next, 1));
                (max.max(1), val)
            },
        );

        let max = max as f32;

        counts_by_value
            .iter()
            .map(|(val, count)| (*val, (*count as f32) / max))
            .collect()
    }
}

impl<T: Roll> Roll for &[&T] {
    fn roll(&self, rng: &mut impl TurboRand) -> u8 {
        if self.is_empty() {
            1
        } else {
            self.iter().map(|v| v.roll(rng)).sum()
        }
    }
}
