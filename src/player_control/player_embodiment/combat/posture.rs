use crate::combat::Constitution;
use crate::movement::general_movement::Walking;
use crate::player_control::player_embodiment::combat::{
    AttackCommitment, PlayerCombatKind, PlayerCombatState,
};
use bevy::prelude::*;

pub(crate) fn update_posture(
    time: Res<Time>,
    mut player: Query<(&mut PlayerCombatState, &mut Constitution, &Walking)>,
) {
    for (mut combat_state, mut constitution, walking) in player.iter_mut() {
        if constitution.is_posture_broken() {
            combat_state.force_use_next_kind(PlayerCombatKind::PostureBroken);
            combat_state.commitment = AttackCommitment::Committed;
            constitution.mark_broken_posture_as_handled();
        }
        let posture_recovery_time = if walking.sprinting {
            None
        } else {
            match combat_state.kind {
                PlayerCombatKind::Idle => Some(0.7),
                PlayerCombatKind::Attack(_) => None,
                PlayerCombatKind::Block => Some(0.7),
                PlayerCombatKind::Deflected => None,
                PlayerCombatKind::PostureBroken => None,
                PlayerCombatKind::Hurt => None,
            }
        };
        let Some(posture_recovery_time) = posture_recovery_time else {
            continue;
        };

        let time_since_hurt_or_block = 1.8;
        if combat_state.time_in_state > posture_recovery_time
            && combat_state.time_since_sprint > posture_recovery_time
            && combat_state.time_since_hurt_or_block > time_since_hurt_or_block
        {
            let factor = if combat_state.kind == PlayerCombatKind::Block {
                2.0
            } else {
                1.0
            };
            constitution.recover_posture(time.delta_seconds() * factor);
        }
    }
}
