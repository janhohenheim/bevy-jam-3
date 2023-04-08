use crate::combat::{Combatant, Constitution};
use crate::player_control::player_embodiment::combat::collision::{
    BlockedByPlayerEvent, DeflectedByPlayerEvent, PlayerHurtEvent,
};
use crate::player_control::player_embodiment::Player;
use bevy::prelude::*;

pub fn handle_hurt_events(
    mut hurt_events: EventReader<PlayerHurtEvent>,
    mut players: Query<(&mut Constitution,), With<Player>>,
) {
    for attack in hurt_events.iter() {
        for (mut constitution,) in players.iter_mut() {
            constitution.take_full_damage(attack);
            // TODO: get knocked back
        }
    }
}

pub fn handle_block_events(
    mut block_events: EventReader<BlockedByPlayerEvent>,
    mut players: Query<(&mut Constitution,), With<Player>>,
) {
    for attack in block_events.iter() {
        for (mut constitution,) in players.iter_mut() {
            constitution.take_posture_damage(attack);
            // TODO: get knocked back
        }
    }
}

pub fn handle_deflect_events(
    mut deflect_events: EventReader<DeflectedByPlayerEvent>,
    mut players: Query<(&mut Constitution,), (With<Player>, Without<Combatant>)>,
    mut enemies: Query<(&mut Constitution,), (With<Combatant>, Without<Player>)>,
) {
    for event in deflect_events.iter() {
        for (mut constitution,) in players.iter_mut() {
            constitution.take_posture_damage(&event.attack); // TODO: scale down
                                                             // If this fails, we are deflecting a projectile
            if let Ok((mut enemy_constitution,)) = enemies.get_mut(event.attacker) {
                enemy_constitution.take_posture_damage(&event.attack); // TODO: scale with repeated deflects
            }
        }
    }
}
