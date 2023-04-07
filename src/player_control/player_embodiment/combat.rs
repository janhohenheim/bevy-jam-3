use crate::level_instantiation::spawning::AnimationEntityLink;
use crate::player_control::player_embodiment::PlayerAction;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_mod_sysfail::sysfail;
pub use components::*;
use leafwing_input_manager::prelude::*;
use std::time::Duration;

mod components;

pub fn attack(
    mut players: Query<(
        &ActionState<PlayerAction>,
        &mut PlayerCombatState,
        &PlayerCombatAnimations,
    )>,
) {
    for (actions, mut combat_state, animations) in players.iter_mut() {
        if actions.just_released(PlayerAction::Attack) {
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
            match combat_state.commitment {
                AttackCommitment::Cancellable => combat_state.use_next_kind(desired_state),
                AttackCommitment::InBufferPeriod => {
                    combat_state.buffer = Some(desired_state);
                }
                AttackCommitment::Committed => {}
            }
        }
    }
}

pub fn update_states(
    time: Res<Time>,
    mut players: Query<(&mut PlayerCombatState, &PlayerCombatAnimations)>,
    animations: Res<Assets<AnimationClip>>,
) {
    for (mut combat_state, animation_handles) in players.iter_mut() {
        combat_state.time_in_state += time.delta_seconds();
        if combat_state.kind == PlayerCombatKind::Idle {
            combat_state.commitment = AttackCommitment::Cancellable;
            continue;
        }
        let animation = combat_state.kind.get_animation(animation_handles);
        if let Some(animation_clip) = animations.get(&animation.handle) {
            let time_fraction = combat_state.time_in_state / animation_clip.duration();
            if time_fraction > 1.0 {
                let next_kind = combat_state.buffer.take().unwrap_or(PlayerCombatKind::Idle);
                combat_state.use_next_kind(next_kind);
            } else if time_fraction < animation.early_cancel_end {
                combat_state.commitment = AttackCommitment::Cancellable;
            } else if time_fraction > animation.early_cancel_end
                && time_fraction < animation.buffer_start
            {
                combat_state.commitment = AttackCommitment::Committed;
            } else if time_fraction > animation.buffer_start
                && time_fraction < animation.late_cancel_start
            {
                combat_state.commitment = AttackCommitment::InBufferPeriod;
            } else if time_fraction > animation.late_cancel_start {
                combat_state.commitment = AttackCommitment::Cancellable;
            }
        }
    }
}

#[sysfail(log(level = "error"))]
pub fn play_animations(
    mut players: Query<(
        &PlayerCombatState,
        &PlayerCombatAnimations,
        &AnimationEntityLink,
    )>,
    mut animation_players: Query<&mut AnimationPlayer>,
) -> Result<()> {
    for (combat_state, animations, animation_entity_link) in players.iter_mut() {
        let mut animation_player = animation_players.get_mut(animation_entity_link.0).context(
            "Animation entity link points to an entity that does not have an animation player",
        )?;
        let animation = combat_state.kind.get_animation(animations).handle.clone();
        animation_player.play_with_transition(animation, Duration::from_secs_f32(0.1));
    }
    Ok(())
}
