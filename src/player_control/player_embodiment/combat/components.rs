use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Bundle)]
pub struct PlayerCombatBundle {
    pub player_combat: PlayerCombatState,
    pub player_combat_animations: PlayerCombatAnimations,
}

#[derive(Debug, Clone, Copy, Component, Reflect, FromReflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct PlayerCombatState {
    pub kind: PlayerCombatKind,
    pub buffer: Option<PlayerCombatKind>,
    pub commitment: AttackCommitment,
    pub time_in_state: f32,
    pub started_animation: bool,
}

impl PlayerCombatState {
    pub fn use_next_kind(&mut self, kind: PlayerCombatKind) {
        *self = Self { kind, ..default() };
    }
}

#[derive(
    Debug, Clone, Copy, Reflect, FromReflect, Serialize, Deserialize, Default, Eq, PartialEq,
)]
#[reflect(Serialize, Deserialize)]
pub enum AttackCommitment {
    Cancellable,
    InBufferPeriod,
    #[default]
    Committed,
}

#[derive(Debug, Clone, Component, Reflect, FromReflect, Default)]
#[reflect(Component)]
pub struct PlayerCombatAnimations {
    pub idle: PlayerCombatAnimation,
    pub attacks: [PlayerCombatAnimation; 3],
    pub block: PlayerCombatAnimation,
    pub hurt: PlayerCombatAnimation,
    pub parried: PlayerCombatAnimation,
    pub perfect_parried: PlayerCombatAnimation,
    pub posture_broken: PlayerCombatAnimation,
}

#[derive(Debug, Clone, Component, Reflect, FromReflect, Default)]
#[reflect(Component)]
pub struct PlayerCombatAnimation {
    pub handle: Handle<AnimationClip>,
    pub early_cancel_end: f32,
    pub late_cancel_start: f32,
    pub buffer_start: f32,
}

impl PlayerCombatAnimation {
    pub fn with_defaults(handle: Handle<AnimationClip>) -> Self {
        Self {
            handle,
            early_cancel_end: 0.1,
            late_cancel_start: 0.9,
            buffer_start: 0.7,
        }
    }
}
#[derive(
    Debug, Clone, Copy, Reflect, FromReflect, Serialize, Deserialize, Default, Eq, PartialEq,
)]
#[reflect(Serialize, Deserialize)]
pub enum PlayerCombatKind {
    #[default]
    Idle,
    Attack(u16),
    Block,
    Parried,
    PerfectParried,
    PostureBroken,
    Hurt,
}

impl PlayerCombatKind {
    pub fn get_animation<'a>(
        &self,
        animations: &'a PlayerCombatAnimations,
    ) -> &'a PlayerCombatAnimation {
        match self {
            PlayerCombatKind::Idle => &animations.idle,
            PlayerCombatKind::Attack(attack) => &animations.attacks[*attack as usize],
            PlayerCombatKind::Block => &animations.block,
            PlayerCombatKind::Hurt => &animations.hurt,
            PlayerCombatKind::Parried => &animations.parried,
            PlayerCombatKind::PerfectParried => &animations.perfect_parried,
            PlayerCombatKind::PostureBroken => &animations.posture_broken,
        }
    }
}
