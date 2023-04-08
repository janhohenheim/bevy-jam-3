use crate::combat::{Attack, Constitution};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Bundle)]
pub struct PlayerCombatBundle {
    pub player_combat: PlayerCombatState,
    pub player_combat_animations: PlayerCombatAnimations,
    pub player_attacks: PlayerAttacks,
    pub constitution: Constitution,
    pub block_history: BlockHistory,
}

#[derive(Debug, Clone, Copy, Component, Reflect, FromReflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct PlayerCombatState {
    pub kind: PlayerCombatKind,
    pub buffer: Option<PlayerCombatKind>,
    pub commitment: AttackCommitment,
    pub time_in_state: f32,
    pub time_since_hit: f32,
    pub time_since_sprint: f32,
    pub started_animation: bool,
}

#[derive(Debug, Clone, Component, Reflect, FromReflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct BlockHistory(Vec<BlockHistoryEntry>);

impl BlockHistory {
    pub fn push(&mut self) {
        self.0.push(default())
    }

    pub fn count_younger_than(&self, time: f32) -> usize {
        self.0
            .iter()
            .rev()
            .take_while(|entry| entry.time < time)
            .count()
    }

    pub fn age(&mut self, dt: f32) {
        for entry in self.0.iter_mut() {
            entry.time += dt;
        }
    }

    pub fn remove_older_than(&mut self, time: f32) {
        self.0.retain(|entry| entry.time < time);
    }

    pub fn mark_last_as_deflect(&mut self) {
        if let Some(entry) = self.0.last_mut() {
            entry.deflect = true;
        }
    }

    pub fn current_deflect_streak(&self) -> usize {
        self.0
            .iter()
            .rev()
            .take_while(|entry| entry.deflect)
            .count()
    }
}

#[derive(Debug, Clone, Copy, Component, Reflect, FromReflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct BlockHistoryEntry {
    pub time: f32,
    pub deflect: bool,
}

impl PlayerCombatState {
    pub fn force_use_next_kind(&mut self, kind: PlayerCombatKind) {
        if self.kind != kind {
            *self = Self {
                kind,
                time_since_sprint: self.time_since_sprint,
                time_since_hit: self.time_since_hit,
                ..default()
            };
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

    pub fn update_timers(&mut self, dt: f32) {
        self.time_in_state += dt;
        self.time_since_hit += dt;
        self.time_since_sprint += dt;
    }
}

#[derive(
    Debug, Clone, Copy, Reflect, FromReflect, Serialize, Deserialize, Default, Eq, PartialEq,
)]
#[reflect(Serialize, Deserialize)]
pub enum AttackCommitment {
    #[default]
    EarlyCancellable,
    LateCancellable,
    InBufferPeriod,
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
    pub deflected: PlayerCombatAnimation,
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
    Deflected,
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
            PlayerCombatKind::Deflected => &animations.deflected,
            PlayerCombatKind::PostureBroken => &animations.posture_broken,
        }
    }

    pub fn get_attack(self, attacks: &PlayerAttacks) -> Option<Attack> {
        match self {
            PlayerCombatKind::Attack(attack) => Some(attacks.attacks[attack as usize].clone()),
            _ => None,
        }
    }

    pub fn is_attack(self) -> bool {
        matches!(self, PlayerCombatKind::Attack(_))
    }
}

#[derive(Debug, Clone, Component, Reflect, FromReflect, Default)]
#[reflect(Component)]
pub struct PlayerAttacks {
    pub attacks: [Attack; 3],
}
