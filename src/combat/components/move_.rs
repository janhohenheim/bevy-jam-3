use crate::combat::{CombatCondition, EnemyCombatState};
use bevy::prelude::*;
pub use melee_attack_fn::*;
pub use motion_fn::*;
pub use projectile_attack_fn::*;
use std::fmt::Debug;

mod melee_attack_fn;
mod motion_fn;
mod projectile_attack_fn;

#[derive(Debug, Clone, Default)]
pub struct Move {
    pub(crate) name: Option<String>,
    pub(crate) metadata: MoveMetadata,
    pub(crate) functions: MoveFunctions,
}

#[derive(Debug, Clone, Default)]
pub struct MoveMetadata {
    pub duration: MoveDuration,
    pub animation: Option<Handle<AnimationClip>>,
    pub state: EnemyCombatState,
}

#[derive(Debug, Clone, Default)]
pub struct MoveFunctions {
    pub motion_fn: Option<Box<dyn MotionFn>>,
    pub melee_attack_fn: Option<Box<dyn MeleeAttackFn>>,
    pub projectile_attack_fn: Option<Box<dyn ProjectileAttackFn>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MoveDuration {
    Fixed(f32),
    Animation,
    Instant,
    While(CombatCondition),
    Until(CombatCondition),
}
impl Default for MoveDuration {
    fn default() -> Self {
        Self::Fixed(0.0)
    }
}

#[derive(Debug, Clone)]
pub struct ReadMoveMetadataEvent {
    pub source: Entity,
    pub move_: MoveMetadata,
}

#[derive(Debug, Clone)]
pub struct ExecuteMoveFunctionsEvent {
    pub source: Entity,
    pub move_: MoveFunctions,
    pub duration: MoveDuration,
}
