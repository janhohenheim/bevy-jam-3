use crate::combat::{Attack, Combatant, Constitution};
use crate::player_control::player_embodiment::combat::collision::{
    BlockedByPlayerEvent, DeflectedByPlayerEvent, PlayerHurtEvent,
};
use crate::player_control::player_embodiment::combat::BlockHistory;
use crate::player_control::player_embodiment::Player;
use bevy::prelude::*;
use bevy_rapier3d::prelude::{ExternalImpulse, ReadMassProperties};

pub fn handle_hurt_events(
    mut hurt_events: EventReader<PlayerHurtEvent>,
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
    for attack in hurt_events.iter() {
        for (mut constitution, mut impulse, mass, transform) in players.iter_mut() {
            constitution.take_full_damage(attack);
            impulse.impulse += attack.knockback * transform.back() * mass.0.mass;
        }
    }
}

pub fn handle_block_events(
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
            let factor = 0.75;
            impulse.impulse += attack.knockback * factor * transform.back() * mass.0.mass;
        }
    }
}

pub fn handle_deflect_events(
    mut deflect_events: EventReader<DeflectedByPlayerEvent>,
    mut players: Query<
        (
            &mut Constitution,
            &BlockHistory,
            &mut ExternalImpulse,
            &ReadMassProperties,
            &Transform,
        ),
        (With<Player>, Without<Combatant>),
    >,
    mut enemies: Query<(&mut Constitution,), (With<Combatant>, Without<Player>)>,
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
                let streak = block_history.current_deflect_streak() as f32;
                let factor = 1.0 + (streak - 1.0) * 0.75;
                let base_posture_damage = 30.;
                let attack = Attack::new(format!(
                    "Player deflecting attack: \"{}\"",
                    event.attack.name
                ))
                .with_posture_damage(base_posture_damage * factor);
                enemy_constitution.take_posture_damage(&attack);
            }
            let factor = 0.6;
            impulse.impulse += event.attack.knockback * factor * transform.back() * mass.0.mass;
        }
    }
}
