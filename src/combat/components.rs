use crate::movement::general_movement::ManualRotation;
use bevy::prelude::*;
use bevy::utils::HashMap;
pub use condition::*;
pub use move_::*;
use serde::{Deserialize, Serialize};

mod condition;
mod move_;

#[derive(Debug, Clone, Bundle)]
pub struct CombatBundle {
    combatant: Combatant,
    combatant_state: CombatantState,
    condition_tracker: ConditionTracker,
    move_metadata: MoveMetadata,
    manual_rotation: ManualRotation,
    constitution: Constitution,
}

impl CombatBundle {
    pub fn new(combatant: Combatant, constitution: Constitution) -> Self {
        Self {
            combatant,
            constitution,
            combatant_state: default(),
            condition_tracker: default(),
            move_metadata: default(),
            manual_rotation: default(),
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
    pub time_since_last_animation: f32,
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

    pub fn current_choreography(&self) -> Option<&Choreography> {
        self.current
            .as_ref()
            .map(|current| &self.choreographies[current.choreography])
    }

    pub fn current_move(&self) -> Option<&Move> {
        self.current.as_ref().and_then(|current| {
            self.choreographies
                .get(current.choreography)
                .and_then(|choreography| choreography.moves.get(current.move_))
        })
    }
}

#[derive(Debug, Clone, Component, Reflect, FromReflect)]
#[reflect(Component)]
pub struct Constitution {
    pub health: f32,
    pub max_health: f32,
    pub posture: f32,
    pub max_posture: f32,
}

impl Constitution {
    pub fn from_max_health_and_posture(max_health: f32, max_posture: f32) -> Self {
        Self {
            health: max_health,
            max_health,
            posture: 0.0,
            max_posture,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0.0
    }

    pub fn is_posture_broken(&self) -> bool {
        self.posture > self.max_posture
    }
}

impl Default for Constitution {
    fn default() -> Self {
        Self::from_max_health_and_posture(100.0, 100.0)
    }
}

#[derive(Debug, Component, Clone, Deref, DerefMut)]
pub struct ParentToHitboxLink(pub Entity);

#[derive(Debug, Component, Clone, Deref, DerefMut)]
pub struct HitboxToParentLink(pub Entity);

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Tendency {
    pub choreography: usize,
    pub weight: f32,
    pub condition: CombatCondition,
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
    pub(crate) animation_duration: Option<f32>,
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

#[derive(
    Debug, Component, Clone, Copy, PartialEq, Reflect, FromReflect, Serialize, Deserialize,
)]
#[reflect(Component, Serialize, Deserialize)]
pub struct AttackHitbox {
    pub(crate) active: bool,
    pub(crate) attack: Attack,
}

impl AttackHitbox {
    pub fn from_attack(attack: Attack) -> Self {
        Self {
            attack,
            ..default()
        }
    }
}

impl Default for AttackHitbox {
    fn default() -> Self {
        Self {
            active: true,
            attack: default(),
        }
    }
}

#[derive(
    Debug, Component, Clone, Copy, PartialEq, Reflect, FromReflect, Serialize, Deserialize, Default,
)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Attack {
    pub(crate) damage: f32,
    pub(crate) knockback: f32,
}

#[derive(
    Debug, Component, Clone, Copy, PartialEq, Default, Reflect, FromReflect, Serialize, Deserialize,
)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Projectile;
