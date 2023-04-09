use crate::util::trait_extension::F32Ext;
use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CombatCondition {
    PlayerDistanceUnder(f32),
    PlayerDistanceOver(f32),
    Grounded,
    #[allow(dead_code)]
    HasLineOfSight,
    None,
    #[allow(dead_code)]
    Not(Box<CombatCondition>),
    And(Vec<CombatCondition>),
    #[allow(dead_code)]
    Or(Vec<CombatCondition>),
}

impl Default for CombatCondition {
    fn default() -> Self {
        Self::PlayerDistanceUnder(0.0)
    }
}

#[derive(Debug, Component, Clone, PartialEq, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub(crate) struct ConditionTracker {
    pub(crate) player_direction: Vec3,
    pub(crate) line_of_sight_direction: Vec3,
    pub(crate) has_line_of_sight: bool,
    pub(crate) grounded: bool,
}

impl ConditionTracker {
    pub(crate) fn all(&self, conditions: &[CombatCondition]) -> bool {
        conditions.iter().all(|condition| self.fulfilled(condition))
    }

    pub(crate) fn any(&self, conditions: &[CombatCondition]) -> bool {
        conditions.iter().any(|condition| self.fulfilled(condition))
    }

    pub(crate) fn fulfilled(&self, condition: &CombatCondition) -> bool {
        match condition {
            CombatCondition::PlayerDistanceUnder(distance) => {
                self.player_direction.length_squared() < distance.squared() + 1e-5
            }
            CombatCondition::PlayerDistanceOver(distance) => {
                self.player_direction.length_squared() > distance.squared() - 1e-5
            }
            CombatCondition::HasLineOfSight => self.has_line_of_sight,
            CombatCondition::Grounded => self.grounded,
            CombatCondition::Not(condition) => !self.fulfilled(condition),
            CombatCondition::And(conditions) => self.all(conditions),
            CombatCondition::Or(conditions) => self.any(conditions),
            CombatCondition::None => true,
        }
    }
}
