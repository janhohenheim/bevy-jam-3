use crate::combat::collision::DeflectedByEnemyEvent;
use crate::combat::{Attack, Constitution, Enemy};
use crate::player_control::player_embodiment::combat::collision::{
    BlockedByPlayerEvent, DeflectedByPlayerEvent, PlayerHurtEvent,
};
use crate::player_control::player_embodiment::combat::{
    AttackCommitment, BlockHistory, PlayerCombatKind, PlayerCombatState,
};
use crate::player_control::player_embodiment::Player;
use crate::world_interaction::side_effects::{SideEffect, SideEffects};
use bevy::prelude::*;
use bevy_rapier3d::prelude::{ExternalImpulse, ReadMassProperties};

pub(crate) fn handle_hurt_events(
    mut hurt_events: EventReader<PlayerHurtEvent>,
    mut players: Query<
        (
            &mut PlayerCombatState,
            &mut Constitution,
            &mut ExternalImpulse,
            &ReadMassProperties,
            &Transform,
        ),
        With<Player>,
    >,
) {
    for attack in hurt_events.iter() {
        for (mut combat_state, mut constitution, mut impulse, mass, transform) in players.iter_mut()
        {
            let factor = if combat_state.kind == PlayerCombatKind::PostureBroken {
                2.0
            } else {
                1.0
            };
            let attack = attack
                .0
                .clone()
                .with_health_damage(attack.health_damage * factor);
            constitution.take_full_damage(&attack);
            combat_state.force_use_next_kind(PlayerCombatKind::Hurt);
            combat_state.commitment = AttackCommitment::Committed;
            impulse.impulse += attack.knockback * transform.back() * mass.0.mass;
        }
    }
}

pub(crate) fn handle_block_events(
    mut block_events: EventReader<BlockedByPlayerEvent>,
    mut players: Query<
        (
            &mut Constitution,
            &mut ExternalImpulse,
            &ReadMassProperties,
            &Transform,
        ),
        With<Player>,
    >,
) {
    for attack in block_events.iter() {
        for (mut constitution, mut impulse, mass, transform) in players.iter_mut() {
            constitution.take_posture_damage(attack);
            let factor = 0.5;
            impulse.impulse += attack.knockback * factor * transform.back() * mass.0.mass;
        }
    }
}

pub(crate) fn handle_deflect_events(
    mut deflect_events: EventReader<DeflectedByPlayerEvent>,
    mut players: Query<
        (
            &mut Constitution,
            &BlockHistory,
            &mut ExternalImpulse,
            &ReadMassProperties,
            &Transform,
        ),
        (With<Player>, Without<Enemy>),
    >,
    mut enemies: Query<(&mut Constitution,), (With<Enemy>, Without<Player>)>,
    side_effects: Res<SideEffects>,
) {
    for event in deflect_events.iter() {
        for (mut constitution, block_history, mut impulse, mass, transform) in players.iter_mut() {
            constitution.take_posture_damage_deflecting(&event.attack);
            assert_ne!(
                block_history.current_deflect_streak(),
                0,
                "Deflecting something, but the block history reports no deflect streak"
            );
            // If this fails, we are deflecting a projectile
            if let Ok((mut enemy_constitution,)) = enemies.get_mut(event.attacker) {
                let streak = block_history.current_deflect_streak();
                let side_effect = side_effects.get_factored(SideEffect::DeflectPostureDamage, 0.3);
                let factor = 1.0 + (streak - 1) as f32 * 0.5 * side_effect;
                let base_posture_damage = 8.;
                let attack = Attack::new(format!(
                    "Player deflecting attack: \"{}\"",
                    event.attack.name
                ))
                .with_posture_damage(base_posture_damage * factor);
                enemy_constitution.take_posture_damage(&attack);
            }
            let factor = 0.3;
            impulse.impulse += event.attack.knockback * factor * transform.back() * mass.0.mass;
        }
    }
}

pub(crate) fn handle_enemy_deflect_events(
    mut attacks: EventReader<DeflectedByEnemyEvent>,
    mut players: Query<(&mut PlayerCombatState,)>,
) {
    for _attack in attacks.iter() {
        for (mut combat_state,) in players.iter_mut() {
            combat_state.force_use_next_kind(PlayerCombatKind::Deflected);
            combat_state.commitment = AttackCommitment::Committed;
        }
    }
}
