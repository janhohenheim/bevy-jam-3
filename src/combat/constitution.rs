use crate::combat::{Constitution, Enemy, EnemyCombatState};
use bevy::prelude::*;
use bevy::utils::HashMap;

pub(crate) fn update_posture(
    time: Res<Time>,
    mut enemies: Query<(&mut Enemy, &EnemyCombatState, &mut Constitution)>,
) {
    for (mut enemy, combat_state, mut constitution) in enemies.iter_mut() {
        if constitution.is_posture_broken() && *combat_state != EnemyCombatState::Dying {
            enemy.break_posture();
            constitution.mark_broken_posture_as_handled();
            continue;
        }
        let posture_recovery_time = match *combat_state {
            EnemyCombatState::OnGuard => Some(0.7),
            EnemyCombatState::Deathblow => None,
            EnemyCombatState::Vulnerable => None,
            EnemyCombatState::HyperArmor => None,
            EnemyCombatState::Dying => None,
        };
        let Some(posture_recovery_time) = posture_recovery_time else {
            continue;
        };

        let time_since_hurt_or_block = 1.5;
        if enemy.time_since_last_move > posture_recovery_time
            && enemy.time_since_hurt_or_block > time_since_hurt_or_block
        {
            constitution.recover_posture(time.delta_seconds());
        }
    }
}

pub(crate) fn handle_death(
    time: Res<Time>,
    mut commands: Commands,
    mut enemies: Query<(Entity, &mut Enemy, &EnemyCombatState, &mut Constitution)>,
    mut marked_for_death: Local<HashMap<Entity, f32>>,
) {
    for (entity, mut enemy, combat_state, mut constitution) in enemies.iter_mut() {
        if !marked_for_death.contains_key(&entity)
            && !(constitution.is_dead()
                || enemy.is_dead
                || *combat_state == EnemyCombatState::Dying)
        {
            return;
        }
        warn!("ded");
        let time_since_death = marked_for_death.entry(entity).or_insert(0.0);
        enemy.die();
        constitution.break_posture();

        const TIME_TO_DESPAWN: f32 = 4.0;
        if *time_since_death > TIME_TO_DESPAWN {
            commands.entity(entity).despawn_recursive();
            marked_for_death.remove(&entity);
        } else {
            *time_since_death += time.delta_seconds();
        }
    }
}
