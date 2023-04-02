use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Bundle)]
pub struct CombatBundle {
    combatant: Combatant,
    combatant_state: CombatantState,
    condition_tracker: ConditionTracker,
}

impl CombatBundle {
    pub fn new(combatant: Combatant) -> Self {
        Self {
            combatant,
            combatant_state: default(),
            condition_tracker: default(),
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

#[derive(Debug, Clone, Default)]
pub struct Move {
    pub duration: MoveDuration,
    pub animation: Option<Handle<AnimationClip>>,
    pub state: CombatantState,
    pub translation_fn: Option<Box<dyn TranslationFn<Output = Vec3>>>,
}

impl Debug for dyn TranslationFn<Output = Vec3> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TranslationFn").finish()
    }
}

pub trait TranslationFn: Fn(TranslationFnInput) -> Vec3 + Send + Sync {
    fn clone_box<'a>(&self) -> Box<dyn TranslationFn<Output = Vec3> + 'a>
    where
        Self: 'a;
}
impl<F> TranslationFn for F
where
    F: Fn(TranslationFnInput) -> Vec3 + Send + Sync + Clone,
{
    fn clone_box<'a>(&self) -> Box<dyn TranslationFn<Output = Vec3> + 'a>
    where
        Self: 'a,
    {
        Box::new(self.clone())
    }
}

impl<'a> Clone for Box<dyn TranslationFn<Output = Vec3> + 'a> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}

#[derive(Debug, Clone, Default)]
pub struct TranslationFnInput {
    pub time: f32,
    pub transform: Transform,
    pub start_transform: Transform,
    pub player_direction: Vec3,
    pub start_player_direction: Vec3,
    pub has_line_of_sight: bool,
    pub line_of_sight_path: Vec<Vec3>,
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
