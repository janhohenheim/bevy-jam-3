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
    pub fn force_use_next_kind(&mut self, kind: PlayerCombatKind) {
        if self.kind != kind {
            *self = Self { kind, ..default() };
        }
    }

    pub fn try_use_next_kind(
        &mut self,
        kind: PlayerCombatKind,
        early_cancel_guard: impl Fn(PlayerCombatKind) -> bool,
    ) {
        match self.commitment {
            AttackCommitment::EarlyCancellable => {
                if early_cancel_guard(self.kind) {
                    self.force_use_next_kind(kind)
                }
            }
            AttackCommitment::LateCancellable => {
                self.force_use_next_kind(kind);
            }
            AttackCommitment::InBufferPeriod => {
                self.buffer = Some(kind);
            }
            AttackCommitment::Committed => {}
        }
    }

    pub fn do_not_block_early_cancel(_kind: PlayerCombatKind) -> bool {
        true
    }
}

#[derive(
    Debug, Clone, Copy, Reflect, FromReflect, Serialize, Deserialize, Default, Eq, PartialEq,
)]
#[reflect(Serialize, Deserialize)]
pub enum AttackCommitment {
    EarlyCancellable,
    LateCancellable,
    InBufferPeriod,
    #[default]
    Committed,
}

impl AttackCommitment {
    pub fn is_cancellable(self) -> bool {
        matches!(
            self,
            AttackCommitment::EarlyCancellable | AttackCommitment::LateCancellable
        )
    }
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
    pub cancellation_times: CancellationTimes,
}

impl PlayerCombatAnimation {
    pub fn with_defaults(handle: Handle<AnimationClip>) -> Self {
        Self {
            handle,
            ..default()
        }
    }

    pub fn without_early_cancel(handle: Handle<AnimationClip>) -> Self {
        Self {
            cancellation_times: CancellationTimes::Periodic(PeriodicCancellationTimes {
                early_cancel_end: 0.0,
                ..default()
            }),
            ..Self::with_defaults(handle)
        }
    }

    pub fn always_cancellable(handle: Handle<AnimationClip>) -> Self {
        Self {
            cancellation_times: CancellationTimes::Always,
            ..Self::with_defaults(handle)
        }
    }
}

#[derive(Debug, Clone, Copy, Reflect, FromReflect)]
pub enum CancellationTimes {
    Always,
    Periodic(PeriodicCancellationTimes),
}

impl Default for CancellationTimes {
    fn default() -> Self {
        Self::Periodic(default())
    }
}

#[derive(Debug, Clone, Copy, Reflect, FromReflect)]
pub struct PeriodicCancellationTimes {
    pub early_cancel_end: f32,
    pub late_cancel_start: f32,
    pub buffer_start: f32,
}

impl Default for PeriodicCancellationTimes {
    fn default() -> Self {
        Self {
            early_cancel_end: 0.2,
            late_cancel_start: 0.85,
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
    pub fn get_animation(self, animations: &PlayerCombatAnimations) -> &PlayerCombatAnimation {
        match self {
            PlayerCombatKind::Idle => &animations.idle,
            PlayerCombatKind::Attack(attack) => &animations.attacks[attack as usize],
            PlayerCombatKind::Block => &animations.block,
            PlayerCombatKind::Hurt => &animations.hurt,
            PlayerCombatKind::Parried => &animations.parried,
            PlayerCombatKind::PerfectParried => &animations.perfect_parried,
            PlayerCombatKind::PostureBroken => &animations.posture_broken,
        }
    }

    pub fn is_attack(self) -> bool {
        matches!(self, PlayerCombatKind::Attack(_))
    }
}
