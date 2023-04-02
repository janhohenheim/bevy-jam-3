use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Bundle)]
pub struct CombatBundle {
    combatant: Combatant,
    combatant_state: CombatantState,
}

#[derive(Debug, Component, Clone, PartialEq, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub struct Combatant {
    pub choreographies: Vec<Choreography>,
    pub last_choreography: Option<usize>,
    pub current: Option<MoveIndex>,
    pub tendencies: Vec<Tendency>,
    pub time_since_last_move: f32,
}

impl Combatant {
    pub fn is_ready_for_next_choreography(&self) -> bool {
        if self.current.is_some() {
            return false;
        }
        if let Some(last_choreography) = self.last_choreography {
            self.time_since_last_move >= self.choreographies[last_choreography].recovery_time
        } else {
            true
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Reflect, FromReflect)]
pub struct Tendency {
    pub choreography: usize,
    pub weight: f32,
    pub conditions: Vec<TendencyCondition>,
}

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect)]
pub enum TendencyCondition {
    MinDistance(f32),
    MaxDistance(f32),
}

impl Default for TendencyCondition {
    fn default() -> Self {
        Self::MinDistance(0.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Reflect, FromReflect)]
pub struct MoveIndex {
    pub choreography: usize,
    pub move_: usize,
}

#[derive(Debug, Clone, PartialEq, Default, Reflect, FromReflect)]
pub struct Choreography {
    pub moves: Vec<Move>,
    pub recovery_time: f32,
}

#[derive(Debug, Clone, PartialEq, Default, Reflect, FromReflect)]
pub struct Move {
    pub duration: f32,
    pub animation: Handle<AnimationClip>,
    pub state: CombatantState,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MoveEvent {
    pub source: Entity,
    pub move_: Move,
}

#[derive(
    Debug, Component, Clone, Copy, PartialEq, Default, Reflect, FromReflect, Serialize, Deserialize,
)]
#[reflect(Component, Serialize, Deserialize)]
pub enum CombatantState {
    Deathblow,
    Vulnerable,
    #[default]
    OnGuard,
    HyperArmor,
}
