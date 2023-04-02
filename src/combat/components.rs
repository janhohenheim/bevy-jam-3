use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Component, Clone, PartialEq, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub struct Combatant {
    pub choreographies: Vec<Choreography>,
    pub current: Option<MoveIndex>,
    pub time_since_last_move: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Reflect, FromReflect)]
pub struct MoveIndex {
    pub choreography: usize,
    pub move_: usize,
}

#[derive(Debug, Clone, PartialEq, Default, Reflect, FromReflect, Deref, DerefMut)]
pub struct Choreography(pub Vec<Move>);

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
