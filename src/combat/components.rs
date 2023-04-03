use crate::level_instantiation::spawning::objects::GameCollisionGroup;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_rapier3d::prelude::*;
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

#[derive(Debug, Bundle)]
pub struct MeleeAttackBundle {
    pub melee_attack: MeleeAttack,
    pub collider: Collider,
    pub collision_groups: CollisionGroups,
    #[bundle]
    pub spatial_bundle: SpatialBundle,
    pub sensor: Sensor,
    pub active_events: ActiveEvents,
    pub active_collision_types: ActiveCollisionTypes,
}

impl MeleeAttackBundle {
    pub fn from_melee_attack(melee_attack: MeleeAttack) -> Self {
        Self {
            melee_attack,
            ..default()
        }
    }
}

impl Default for MeleeAttackBundle {
    fn default() -> Self {
        Self {
            melee_attack: default(),
            collider: default(),
            spatial_bundle: default(),
            collision_groups: CollisionGroups::new(
                GameCollisionGroup::ATTACK.into(),
                GameCollisionGroup::NONE.into(),
            ),
            sensor: default(),
            active_events: ActiveEvents::COLLISION_EVENTS,
            active_collision_types: ActiveCollisionTypes::DYNAMIC_DYNAMIC,
        }
    }
}

#[derive(Debug, Component, Clone, Default)]
pub struct MeleeAttack {
    pub(crate) damage: f32,
    pub(crate) knockback: f32,
}

#[derive(Debug, Component, Clone, Deref, DerefMut)]
pub struct MeleeAttackLink(pub Entity);

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
