use crate::level_instantiation::spawning::AnimationEntityLink;
use crate::player_control::player_embodiment::PlayerAction;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_mod_sysfail::sysfail;
pub use components::*;
use leafwing_input_manager::prelude::*;
use std::time::Duration;

mod components;
#[cfg(feature = "dev")]
pub mod debug;

pub fn attack(
    mut players: Query<(
        &ActionState<PlayerAction>,
        &mut PlayerCombatState,
        &PlayerCombatAnimations,
    )>,
) {
    for (actions, mut combat_state, animations) in players.iter_mut() {
        if actions.pressed(PlayerAction::Attack) {
            let desired_state = if let PlayerCombatKind::Attack(attack) = combat_state.kind {
                let next_attack = if attack + 1 < animations.attacks.len() as u16 {
                    attack + 1
                } else {
                    0
                };
                PlayerCombatKind::Attack(next_attack)
            } else {
                PlayerCombatKind::Attack(0)
            };
            combat_state.try_use_next_kind(desired_state, |current| !current.is_attack());
        }
    }
}

pub fn block(mut players: Query<(&ActionState<PlayerAction>, &mut PlayerCombatState)>) {
    for (actions, mut combat_state) in players.iter_mut() {
        if actions.pressed(PlayerAction::Block) {
            combat_state.try_use_next_kind(PlayerCombatKind::Block, |current: PlayerCombatKind| {
                current != PlayerCombatKind::Block
            });
            info!("Block pressed");
        } else if actions.just_released(PlayerAction::Block) {
            combat_state.try_use_next_kind(
                PlayerCombatKind::Idle,
                PlayerCombatState::do_not_block_early_cancel,
            );
        }
    }
}

pub fn update_states(
    time: Res<Time>,
    mut players: Query<(&mut PlayerCombatState, &PlayerCombatAnimations)>,
    animation_clips: Res<Assets<AnimationClip>>,
) {
    for (mut combat_state, combat_animations) in players.iter_mut() {
        combat_state.time_in_state += time.delta_seconds();
        let animation = combat_state.kind.get_animation(combat_animations);
        match animation.cancellation_times {
            CancellationTimes::Always => {
                combat_state.commitment = AttackCommitment::EarlyCancellable;
            }
            CancellationTimes::Periodic(cancellation_times) => {
                let Some(animation_clip) = animation_clips.get(&animation.handle) else {continue;};
                let time_fraction = combat_state.time_in_state / animation_clip.duration();
                if time_fraction > 1.0 {
                    let next_kind = combat_state.buffer.take().unwrap_or(PlayerCombatKind::Idle);
                    combat_state.force_use_next_kind(next_kind);
                } else if time_fraction < cancellation_times.early_cancel_end {
                    combat_state.commitment = AttackCommitment::EarlyCancellable;
                } else if time_fraction > cancellation_times.early_cancel_end
                    && time_fraction < cancellation_times.buffer_start
                {
                    combat_state.commitment = AttackCommitment::Committed;
                } else if time_fraction > cancellation_times.buffer_start
                    && time_fraction < cancellation_times.late_cancel_start
                {
                    combat_state.commitment = AttackCommitment::InBufferPeriod;
                } else if time_fraction > cancellation_times.late_cancel_start {
                    combat_state.commitment = AttackCommitment::LateCancellable;
                }
            }
        }
    }
}

#[sysfail(log(level = "error"))]
pub fn play_animations(
    mut players: Query<(
        &mut PlayerCombatState,
        &PlayerCombatAnimations,
        &AnimationEntityLink,
    )>,
    mut animation_players: Query<&mut AnimationPlayer>,
) -> Result<()> {
    for (mut combat_state, animations, animation_entity_link) in players.iter_mut() {
        if combat_state.started_animation {
            continue;
        }
        let mut animation_player = animation_players.get_mut(animation_entity_link.0).context(
            "Animation entity link points to an entity that does not have an animation player",
        )?;
        let animation = combat_state.kind.get_animation(animations);
        animation_player
            .start_with_transition(animation.handle.clone(), Duration::from_secs_f32(0.1));
        if matches!(animation.cancellation_times, CancellationTimes::Always) {
            animation_player.repeat();
        }
        combat_state.started_animation = true;
    }
    Ok(())
}
