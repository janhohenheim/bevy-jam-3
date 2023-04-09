use crate::combat::{Attack, Constitution};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Bundle)]
pub(crate) struct PlayerCombatBundle {
    pub(crate) player_combat: PlayerCombatState,
    pub(crate) player_combat_animations: PlayerCombatAnimations,
    pub(crate) player_attacks: PlayerAttacks,
    pub(crate) constitution: Constitution,
    pub(crate) block_history: BlockHistory,
}

#[derive(Debug, Clone, Copy, Component, Reflect, FromReflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct PlayerCombatState {
    pub(crate) kind: PlayerCombatKind,
    pub(crate) buffer: Option<PlayerCombatKind>,
    pub(crate) commitment: AttackCommitment,
    pub(crate) time_in_state: f32,
    pub(crate) time_since_hurt_or_block: f32,
    pub(crate) time_since_sprint: f32,
    pub(crate) started_animation: bool,
}

#[derive(Debug, Clone, Component, Reflect, FromReflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct BlockHistory(Vec<BlockHistoryEntry>);

impl BlockHistory {
    pub(crate) fn push(&mut self) {
        self.0.push(default())
    }

    pub(crate) fn count_younger_than(&self, time: f32) -> usize {
        self.0
            .iter()
            .rev()
            .take_while(|entry| entry.time < time)
            .count()
    }

    pub(crate) fn age(&mut self, dt: f32) {
        for entry in self.0.iter_mut() {
            entry.time += dt;
        }
    }

    pub(crate) fn remove_older_than(&mut self, time: f32) {
        self.0.retain(|entry| entry.time < time);
    }

    pub(crate) fn mark_last_as_deflect(&mut self) {
        if let Some(entry) = self.0.last_mut() {
            entry.deflect = true;
        }
    }

    pub(crate) fn current_deflect_streak(&self) -> usize {
        self.0
            .iter()
            .rev()
            .take_while(|entry| entry.deflect)
            .count()
    }
}

#[derive(Debug, Clone, Copy, Component, Reflect, FromReflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct BlockHistoryEntry {
    pub(crate) time: f32,
    pub(crate) deflect: bool,
}

impl PlayerCombatState {
    pub(crate) fn force_use_next_kind(&mut self, kind: PlayerCombatKind) {
        if self.kind != kind {
            *self = Self {
                kind,
                time_since_sprint: self.time_since_sprint,
                time_since_hurt_or_block: self.time_since_hurt_or_block,
                ..default()
            };
        }
    }

    pub(crate) fn try_use_next_kind(
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

    pub(crate) fn do_not_block_early_cancel(_kind: PlayerCombatKind) -> bool {
        true
    }

    pub(crate) fn update_timers(&mut self, dt: f32) {
        self.time_in_state += dt;
        self.time_since_hurt_or_block += dt;
        self.time_since_sprint += dt;
    }
}

#[derive(
    Debug, Clone, Copy, Reflect, FromReflect, Serialize, Deserialize, Default, Eq, PartialEq,
)]
#[reflect(Serialize, Deserialize)]
pub(crate) enum AttackCommitment {
    #[default]
    EarlyCancellable,
    LateCancellable,
    InBufferPeriod,
    Committed,
}

impl AttackCommitment {
    pub(crate) fn is_cancellable(self) -> bool {
        matches!(
            self,
            AttackCommitment::EarlyCancellable | AttackCommitment::LateCancellable
        )
    }
}

#[derive(Debug, Clone, Component, Reflect, FromReflect, Default)]
#[reflect(Component)]
pub(crate) struct PlayerCombatAnimations {
    pub(crate) idle: PlayerCombatAnimation,
    pub(crate) attacks: [PlayerCombatAnimation; 3],
    pub(crate) block: PlayerCombatAnimation,
    pub(crate) hurt: PlayerCombatAnimation,
    pub(crate) deflected: PlayerCombatAnimation,
    pub(crate) posture_broken: PlayerCombatAnimation,
}

#[derive(Debug, Clone, Component, Reflect, FromReflect, Default)]
#[reflect(Component)]
pub(crate) struct PlayerCombatAnimation {
    pub(crate) handle: Handle<AnimationClip>,
    pub(crate) cancellation_times: CancellationTimes,
}

impl PlayerCombatAnimation {
    pub(crate) fn with_defaults(handle: Handle<AnimationClip>) -> Self {
        Self {
            handle,
            ..default()
        }
    }

    pub(crate) fn always_cancellable(handle: Handle<AnimationClip>) -> Self {
        Self {
            cancellation_times: CancellationTimes::Always,
            ..Self::with_defaults(handle)
        }
    }
}

#[derive(Debug, Clone, Copy, Reflect, FromReflect)]
pub(crate) enum CancellationTimes {
    Always,
    Periodic(PeriodicCancellationTimes),
}

impl Default for CancellationTimes {
    fn default() -> Self {
        Self::Periodic(default())
    }
}

#[derive(Debug, Clone, Copy, Reflect, FromReflect)]
pub(crate) struct PeriodicCancellationTimes {
    pub(crate) early_cancel_end: f32,
    pub(crate) late_cancel_start: f32,
    pub(crate) buffer_start: f32,
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
pub(crate) enum PlayerCombatKind {
    #[default]
    Idle,
    Attack(u16),
    Block,
    Deflected,
    PostureBroken,
    Hurt,
}

impl PlayerCombatKind {
    pub(crate) fn get_animation(
        self,
        animations: &PlayerCombatAnimations,
    ) -> &PlayerCombatAnimation {
        match self {
            PlayerCombatKind::Idle => &animations.idle,
            PlayerCombatKind::Attack(attack) => &animations.attacks[attack as usize],
            PlayerCombatKind::Block => &animations.block,
            PlayerCombatKind::Hurt => &animations.hurt,
            PlayerCombatKind::Deflected => &animations.deflected,
            PlayerCombatKind::PostureBroken => &animations.posture_broken,
        }
    }

    pub(crate) fn get_attack(self, attacks: &PlayerAttacks) -> Option<Attack> {
        match self {
            PlayerCombatKind::Attack(attack) => Some(attacks.attacks[attack as usize].clone()),
            _ => None,
        }
    }

    pub(crate) fn is_attack(self) -> bool {
        matches!(self, PlayerCombatKind::Attack(_))
    }
}

#[derive(Debug, Clone, Component, Reflect, FromReflect, Default)]
#[reflect(Component)]
pub(crate) struct PlayerAttacks {
    pub(crate) attacks: [Attack; 3],
}
