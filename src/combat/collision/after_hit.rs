use crate::combat::collision::{BlockedByEnemyEvent, DeflectedByEnemyEvent, EnemyHurtEvent};
use crate::combat::{Constitution, Enemy};
use bevy::prelude::*;

pub fn handle_hurt_events(
    mut hurt_events: EventReader<EnemyHurtEvent>,
    mut enemies: Query<(&mut Enemy, &mut Constitution)>,
) {
    for attack in hurt_events.iter() {
        for (mut enemy, mut constitution) in enemies.iter_mut() {
            constitution.take_full_damage(attack);
            enemy.hurt();
        }
    }
}

pub fn handle_block_events(
    mut block_events: EventReader<BlockedByEnemyEvent>,
    mut enemies: Query<(&mut Enemy, &mut Constitution)>,
) {
    for attack in block_events.iter() {
        for (mut enemy, mut constitution) in enemies.iter_mut() {
            constitution.take_posture_damage(attack);
            enemy.block();
        }
    }
}

pub fn handle_deflect_events(
    mut deflect_events: EventReader<DeflectedByEnemyEvent>,
    mut enemies: Query<(&mut Enemy, &mut Constitution)>,
) {
    for attack in deflect_events.iter() {
        for (mut enemy, mut constitution) in enemies.iter_mut() {
            constitution.take_posture_damage(attack); // Enemies don't get any bonus on deflections
            enemy.block();
        }
    }
}
