use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Bundle)]
pub struct CombatBundle {
    combatant: Combatant,
    combatant_state: CombatantState,
    condition_tracker: ConditionTracker,
    move_metadata: MoveMetadata,
}

impl CombatBundle {
    pub fn new(combatant: Combatant) -> Self {
        Self {
            combatant,
            combatant_state: default(),
            condition_tracker: default(),
            move_metadata: default(),
        }
    }
}

#[derive(Debug, Component, Clone, Default)]
pub struct Combatant {
    pub choreographies: Vec<Choreography>,
    pub last_choreography: Option<usize>,
    pub current: Option<CurrentMove>,
    pub tendencies: Vec<Tendency>,
    /// Used to implement e.g. circling around player after a strong boss attack.
    /// Currently does not factor in any conditions.
    pub chained_choreographies: HashMap<usize, usize>,
    pub time_since_last_move: f32,
}

impl Combatant {
    pub fn new(
        choreographies: Vec<Choreography>,
        tendencies: Vec<Tendency>,
        chained_choreographies: HashMap<usize, usize>,
    ) -> Self {
        Self {
            choreographies,
            tendencies,
            chained_choreographies,
            ..default()
        }
    }
    pub fn is_ready_for_next_choreography(&self) -> bool {
        self.current.is_none()
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Tendency {
    pub choreography: usize,
    pub weight: f32,
    pub condition: Condition,
}

#[derive(Debug, Clone, Copy, Default, Reflect, FromReflect)]
pub struct CurrentMove {
    pub choreography: usize,
    pub move_: usize,
    pub start_transform: Transform,
}

#[derive(Debug, Clone, Default)]
pub struct Choreography {
    pub name: String,
    pub moves: Vec<Move>,
}

#[derive(
    Debug, Component, Clone, Copy, PartialEq, Default, Reflect, FromReflect, Serialize, Deserialize,
)]
#[reflect(Component, Serialize, Deserialize)]
pub struct MoveMetadata {
    pub(crate) start_transform: Transform,
    pub(crate) start_player_direction: Vec3,
}

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

impl Debug for dyn ForceFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TranslationFn").finish()
    }
}

pub trait ForceFn: Send + Sync {
    fn call(&self, input: ForceFnInput) -> ForceFnOutput;
    fn clone_box<'a>(&self) -> Box<dyn ForceFn + 'a>
    where
        Self: 'a;
}
impl<F> ForceFn for F
where
    F: Fn(ForceFnInput) -> ForceFnOutput + Send + Sync + Clone,
{
    fn call(&self, input: ForceFnInput) -> ForceFnOutput {
        self(input)
    }

    fn clone_box<'a>(&self) -> Box<dyn ForceFn + 'a>
    where
        Self: 'a,
    {
        Box::new(self.clone())
    }
}

impl<'a> Clone for Box<dyn ForceFn + 'a> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
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
    While(Condition),
    Until(Condition),
}
impl Default for MoveDuration {
    fn default() -> Self {
        Self::Fixed(0.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Condition {
    PlayerDistanceSquaredUnder(f32),
    PlayerDistanceSquaredOver(f32),
    HasLineOfSight,
    Not(Box<Condition>),
    And(Vec<Condition>),
    Or(Vec<Condition>),
}

impl Default for Condition {
    fn default() -> Self {
        Self::PlayerDistanceSquaredUnder(0.0)
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

#[derive(Debug, Component, Clone, PartialEq, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub struct ConditionTracker {
    pub player_direction: Vec3,
    pub line_of_sight_path: Vec<Vec3>,
    pub has_line_of_sight: bool,
    pub active: bool,
}

impl ConditionTracker {
    pub fn all(&self, conditions: &[Condition]) -> bool {
        conditions.iter().all(|condition| self.fulfilled(condition))
    }

    pub fn any(&self, conditions: &[Condition]) -> bool {
        conditions.iter().any(|condition| self.fulfilled(condition))
    }

    pub fn fulfilled(&self, condition: &Condition) -> bool {
        self.active
            && match condition {
                Condition::PlayerDistanceSquaredUnder(distance_squared) => {
                    self.player_direction.length_squared() < *distance_squared
                }
                Condition::PlayerDistanceSquaredOver(distance_squared) => {
                    self.player_direction.length_squared() > *distance_squared
                }
                Condition::HasLineOfSight => self.has_line_of_sight,
                Condition::Not(condition) => !self.fulfilled(condition),
                Condition::And(conditions) => self.all(conditions),
                Condition::Or(conditions) => self.any(conditions),
            }
    }
}
