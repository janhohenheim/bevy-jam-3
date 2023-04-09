use crate::combat::Constitution;
use crate::movement::general_movement::Walking;
use crate::player_control::player_embodiment::combat::{PlayerCombatKind, PlayerCombatState};
use bevy::prelude::*;

pub fn update_posture(
    time: Res<Time>,
    mut player: Query<(&mut PlayerCombatState, &mut Constitution, &Walking)>,
) {
    for (mut combat_state, mut constitution, walking) in player.iter_mut() {
        if constitution.is_posture_broken() {
            combat_state.force_use_next_kind(PlayerCombatKind::PostureBroken);
            constitution.recover_after_posture_broken();
        }
        let posture_recovery_time = if walking.sprinting {
            None
        } else {
            match combat_state.kind {
                PlayerCombatKind::Idle => Some(0.6),
                PlayerCombatKind::Attack(_) => None,
                PlayerCombatKind::Block => Some(1.2),
                PlayerCombatKind::Deflected => None,
                PlayerCombatKind::PostureBroken => None,
                PlayerCombatKind::Hurt => None,
            }
        };

        if let Some(posture_recovery_time) = posture_recovery_time {
            if combat_state.time_in_state > posture_recovery_time
                && combat_state.time_since_sprint > posture_recovery_time
                && combat_state.time_since_hit > posture_recovery_time
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
}
