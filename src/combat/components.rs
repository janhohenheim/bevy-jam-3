use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Component, Clone, PartialEq, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub struct Combatant {
    pub choreographies: Vec<Choreography>,
    pub last_choreography: Option<usize>,
    pub current: Option<MoveIndex>,
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

#[derive(Debug, Clone, PartialEq, Default, Reflect, FromReflect)]
pub struct Tendency {
    pub choreography: usize,
    pub weight: f32,
    pub conditions: Vec<Condition>,
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Reflect, FromReflect)]
pub struct MoveIndex {
    pub choreography: usize,
    pub move_: usize,
}

#[derive(Debug, Clone, PartialEq, Default, Reflect, FromReflect)]
pub struct Choreography {
    pub name: String,
    pub moves: Vec<Move>,
}

#[derive(Debug, Clone, PartialEq, Default, Reflect, FromReflect)]
pub struct Move {
    pub duration: MoveDuration,
    pub animation: Option<Handle<AnimationClip>>,
    pub state: CombatantState,
}

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect)]
pub enum MoveDuration {
    Fixed(f32),
    WhileAll(Vec<Condition>),
    UntilAll(Vec<Condition>),
}
impl Default for MoveDuration {
    fn default() -> Self {
        Self::Fixed(0.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Reflect, FromReflect)]
pub enum Condition {
    PlayerDistanceSquaredUnder(f32),
    PlayerDistanceSquaredOver(f32),
}

impl Default for Condition {
    fn default() -> Self {
        Self::PlayerDistanceSquaredUnder(0.0)
    }
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

#[derive(Debug, Component, Clone, PartialEq, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub struct ConditionTracker {
    pub player_distance_squared: f32,
}

impl ConditionTracker {
    pub fn all(&self, conditions: &[Condition]) -> bool {
        conditions.iter().all(|condition| self.fulfilled(condition))
    }

    pub fn any(&self, conditions: &[Condition]) -> bool {
        conditions.iter().any(|condition| self.fulfilled(condition))
    }

    pub fn fulfilled(&self, condition: &Condition) -> bool {
        match condition {
            Condition::PlayerDistanceSquaredUnder(distance) => {
                self.player_distance_squared < *distance
            }
            Condition::PlayerDistanceSquaredOver(distance) => {
                self.player_distance_squared > *distance
            }
        }
    }
}
