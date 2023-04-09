use crate::combat::{CombatCondition, EnemyCombatState};
use bevy::prelude::*;
pub(crate) use melee_attack_fn::*;
pub(crate) use motion_fn::*;
pub(crate) use projectile_attack_fn::*;
use std::fmt::Debug;

mod melee_attack_fn;
mod motion_fn;
mod projectile_attack_fn;

#[derive(Debug, Clone, Default)]
pub(crate) struct Move {
    pub(crate) name: Option<String>,
    pub(crate) metadata: MoveMetadata,
    pub(crate) functions: MoveFunctions,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct MoveMetadata {
    pub(crate) duration: MoveDuration,
    pub(crate) animation: Option<Handle<AnimationClip>>,
    pub(crate) state: EnemyCombatState,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct MoveFunctions {
    pub(crate) motion_fn: Option<Box<dyn MotionFn>>,
    pub(crate) melee_attack_fn: Option<Box<dyn MeleeAttackFn>>,
    pub(crate) projectile_attack_fn: Option<Box<dyn ProjectileAttackFn>>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum MoveDuration {
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
pub(crate) struct ReadMoveMetadataEvent {
    pub(crate) source: Entity,
    pub(crate) move_: MoveMetadata,
}

#[derive(Debug, Clone)]
pub(crate) struct ExecuteMoveFunctionsEvent {
    pub(crate) source: Entity,
    pub(crate) move_: MoveFunctions,
    pub(crate) duration: MoveDuration,
}
