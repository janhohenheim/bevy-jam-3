use crate::combat::{CombatCondition, CombatantState};
pub use attack_fn::*;
use bevy::prelude::*;
pub use force_fn::*;
use std::fmt::Debug;

mod attack_fn;
mod force_fn;

#[derive(Debug, Clone, Default)]
pub struct Move {
    pub(crate) init: InitMove,
    pub(crate) execute: ExecuteMove,
}

#[derive(Debug, Clone, Default)]
pub struct InitMove {
    pub duration: MoveDuration,
    pub animation: Option<Handle<AnimationClip>>,
    pub state: CombatantState,
}

#[derive(Debug, Clone, Default)]
pub struct ExecuteMove {
    pub force_fn: Option<Box<dyn ForceFn>>,
    pub attack_fn: Option<Box<dyn AttackFn>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MoveDuration {
    Fixed(f32),
    While(CombatCondition),
    Until(CombatCondition),
}
impl Default for MoveDuration {
    fn default() -> Self {
        Self::Fixed(0.0)
    }
}

#[derive(Debug, Clone)]
pub struct InitMoveEvent {
    pub source: Entity,
    pub move_: InitMove,
}

#[derive(Debug, Clone)]
pub struct ExecuteMoveEvent {
    pub source: Entity,
    pub move_: ExecuteMove,
}
