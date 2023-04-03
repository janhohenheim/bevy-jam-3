use crate::combat::{CombatCondition, CombatantState};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
pub use force_fn_trait::*;
use std::fmt::Debug;

mod force_fn_trait;

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
}

#[derive(Debug, Clone, Default)]
pub struct ForceFnInput {
    pub time: f32,
    pub transform: Transform,
    pub start_transform: Transform,
    pub player_direction: Vec3,
    pub start_player_direction: Vec3,
    pub has_line_of_sight: bool,
    pub line_of_sight_path: Vec<Vec3>,
    pub mass: f32,
}

#[derive(Debug, Clone, Default)]
pub struct ForceFnOutput {
    pub force: ExternalForce,
    pub rotation: Option<Quat>,
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
