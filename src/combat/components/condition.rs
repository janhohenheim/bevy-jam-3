use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum CombatCondition {
    PlayerDistanceSquaredUnder(f32),
    PlayerDistanceSquaredOver(f32),
    HasLineOfSight,
    Not(Box<CombatCondition>),
    And(Vec<CombatCondition>),
    Or(Vec<CombatCondition>),
}

impl Default for CombatCondition {
    fn default() -> Self {
        Self::PlayerDistanceSquaredUnder(0.0)
    }
}

#[derive(Debug, Component, Clone, PartialEq, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub struct ConditionTracker {
    pub player_direction: Vec3,
    pub line_of_sight_path: Vec<Vec3>,
    pub has_line_of_sight: bool,
    pub active: bool,
}

impl ConditionTracker {
    pub fn all(&self, conditions: &[CombatCondition]) -> bool {
        conditions.iter().all(|condition| self.fulfilled(condition))
    }

    pub fn any(&self, conditions: &[CombatCondition]) -> bool {
        conditions.iter().any(|condition| self.fulfilled(condition))
    }

    pub fn fulfilled(&self, condition: &CombatCondition) -> bool {
        self.active
            && match condition {
                CombatCondition::PlayerDistanceSquaredUnder(distance_squared) => {
                    self.player_direction.length_squared() < *distance_squared
                }
                CombatCondition::PlayerDistanceSquaredOver(distance_squared) => {
                    self.player_direction.length_squared() > *distance_squared
                }
                CombatCondition::HasLineOfSight => self.has_line_of_sight,
                CombatCondition::Not(condition) => !self.fulfilled(condition),
                CombatCondition::And(conditions) => self.all(conditions),
                CombatCondition::Or(conditions) => self.any(conditions),
            }
    }
}
