use crate::combat::Constitution;
use crate::player_control::player_embodiment::Player;
use crate::world_interaction::room::EnterRoomEvent;
use crate::GameState;
use bevy::prelude::*;
use bevy::utils::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub(crate) mod potions;

pub(crate) fn side_effects_plugin(app: &mut App) {
    app.register_type::<SideEffects>()
        .init_resource::<SideEffects>()
        .add_system(apply_side_effects_to_constitution.in_set(OnUpdate(GameState::Playing)));
}

#[derive(Debug, Clone, Eq, PartialEq, Resource, Reflect, FromReflect, Deref, DerefMut)]
#[reflect(Resource)]
pub(crate) struct SideEffects(HashMap<SideEffect, i32>);

impl SideEffects {
    pub(crate) fn get(&self, side_effect: SideEffect) -> i32 {
        *self.0.get(&side_effect).unwrap()
    }
    pub(crate) fn get_factored(&self, side_effect: SideEffect, factor: f32) -> f32 {
        let value = self.get(side_effect);
        (1. + value as f32 * factor).max(0.01)
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
    HealthDamage,
    AttackPostureDamage,
    AttackKnockback,
    KnockbackResistance,
    MaxPosture,
    PostureRegenRate,
    PostureRegenWait,
    DeflectWindow,
    DeflectPostureDamage,
    BackwardsWalkingSpeed,
    Health,
}

fn apply_side_effects_to_constitution(
    mut enter_room_events: EventReader<EnterRoomEvent>,
    mut players: Query<&mut Constitution, With<Player>>,
    side_effects: Res<SideEffects>,
) {
    for _ in enter_room_events.iter() {
        for mut constitution in players.iter_mut() {
            constitution
                .apply_health_side_effect(side_effects.get_factored(SideEffect::Health, 0.2));
            constitution
                .apply_posture_side_effect(side_effects.get_factored(SideEffect::MaxPosture, 0.2));
            constitution.apply_posture_recovery_side_effect(
                side_effects.get_factored(SideEffect::PostureRegenRate, 0.2),
            );
        }
    }
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
            Self::HealthDamage => "Increase",
            Self::AttackPostureDamage => "Increase",
            Self::MaxPosture => "Increase",
            Self::PostureRegenRate => "Increase",
            Self::PostureRegenWait => "Decrease",
            Self::DeflectWindow => "Increase",
            Self::DeflectPostureDamage => "Increase",
            Self::BackwardsWalkingSpeed => "Increase",
            Self::Health => "Increase",
            Self::AttackKnockback => "Increase",
            Self::KnockbackResistance => "Increase",
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
            Self::HealthDamage => "damage dealt to health by attacking",
            Self::AttackPostureDamage => "damage dealt to posture by attacking",
            Self::MaxPosture => "maximum posture",
            Self::PostureRegenRate => "posture regeneration rate",
            Self::PostureRegenWait => "time before posture regeneration starts",
            Self::DeflectWindow => "time window where blocking an attack will deflect it",
            Self::DeflectPostureDamage => "damage dealt to posture by deflecting an attack",
            Self::BackwardsWalkingSpeed => "walking speed when walking backwards",
            Self::Health => "your health",
            Self::AttackKnockback => "knockback dealt by attacking",
            Self::KnockbackResistance => "resistance to being knocked back",
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
