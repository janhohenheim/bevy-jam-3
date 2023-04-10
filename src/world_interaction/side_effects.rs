use bevy::prelude::*;
use bevy::utils::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub(crate) mod potions;

pub(crate) fn side_effects_plugin(app: &mut App) {
    app.register_type::<SideEffects>()
        .init_resource::<SideEffects>();
}

#[derive(Debug, Clone, Eq, PartialEq, Resource, Reflect, FromReflect, Deref, DerefMut)]
#[reflect(Resource)]
pub(crate) struct SideEffects(HashMap<SideEffect, i32>);

impl SideEffects {
    pub(crate) fn get(&self, side_effect: &SideEffect) -> i32 {
        *self.0.get(side_effect).unwrap()
    }
    pub(crate) fn add_positive(&mut self, side_effect: SideEffect) {
        *self.get_mut(&side_effect).unwrap() += 1;
    }

    pub(crate) fn add_negative(&mut self, side_effect: SideEffect) {
        *self.get_mut(&side_effect).unwrap() -= 1;
    }
}

impl Default for SideEffects {
    fn default() -> Self {
        Self(
            SideEffect::iter()
                .map(|side_effect| (side_effect, 0))
                .collect(),
        )
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect, FromReflect, EnumIter)]
pub(crate) enum SideEffect {
    Size,
    BaseSpeed,
    RunSpeed,
    HealthDamage,
    AttackPostureDamage,
    MaxPosture,
    PostureRegenRate,
    DeflectStaggerTime,
    PostureRegenWait,
    DeflectWindow,
    DeflectPostureDamage,
    BackwardsWalkingSpeed,
    Health,
}

impl SideEffect {
    pub(crate) fn format_positive(self) -> String {
        format!("{} {}", self.positive_descriptor(), self.name())
    }

    pub(crate) fn format_negative(self) -> String {
        format!("{} {}", self.negative_descriptor(), self.name())
    }

    fn positive_descriptor(self) -> &'static str {
        match self {
            Self::Size => "Increase",
            Self::BaseSpeed => "Increase",
            Self::RunSpeed => "Increase",
            Self::HealthDamage => "Increase",
            Self::AttackPostureDamage => "Increase",
            Self::MaxPosture => "Increase",
            Self::PostureRegenRate => "Increase",
            Self::DeflectStaggerTime => "Decrease",
            Self::PostureRegenWait => "Decrease",
            Self::DeflectWindow => "Increase",
            Self::DeflectPostureDamage => "Increase",
            Self::BackwardsWalkingSpeed => "Increase",
            Self::Health => "Increase",
        }
    }

    fn negative_descriptor(self) -> &'static str {
        let positive_descriptor = self.positive_descriptor();
        Self::invert_descriptor(positive_descriptor)
    }

    fn name(self) -> &'static str {
        match self {
            Self::Size => "size",
            Self::BaseSpeed => "walking speed",
            Self::RunSpeed => "running speed",
            Self::HealthDamage => "damage dealt to health by attacking",
            Self::AttackPostureDamage => "damage dealt to posture by attacking",
            Self::MaxPosture => "maximum posture",
            Self::PostureRegenRate => "posture regeneration rate",
            Self::DeflectStaggerTime => "time staggered after being deflected",
            Self::PostureRegenWait => "time before posture regeneration starts",
            Self::DeflectWindow => "time window where blocking an attack will deflect it",
            Self::DeflectPostureDamage => "damage dealt to posture by deflecting an attack",
            Self::BackwardsWalkingSpeed => "walking speed when walking backwards",
            Self::Health => "your health",
        }
    }

    fn invert_descriptor(descriptor: &'static str) -> &'static str {
        match descriptor {
            "Increase" => "Decrease",
            "Decrease" => "Increase",
            _ => panic!("Invalid descriptor"),
        }
    }
}
