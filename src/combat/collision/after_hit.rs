use crate::combat::collision::{BlockedByEnemyEvent, DeflectedByEnemyEvent, EnemyHurtEvent};
use crate::combat::{Constitution, Enemy};
use crate::level_instantiation::spawning::AnimationEntityLink;
use anyhow::Result;
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;

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

#[sysfail(log(level = "error"))]
pub fn handle_block_events(
    mut block_events: EventReader<BlockedByEnemyEvent>,
    mut enemies: Query<(&mut Enemy, &mut Constitution, &AnimationEntityLink)>,
    mut animation_players: Query<&mut AnimationPlayer>,
) -> Result<()> {
    for attack in block_events.iter() {
        for (mut enemy, mut constitution, animation_entity_link) in enemies.iter_mut() {
            constitution.take_posture_damage(attack);
            enemy.block();
            let mut animation_player = animation_players.get_mut(**animation_entity_link)?;
            // Force the animation to restart in case we are already blocking the previous attack
            animation_player.pause();
        }
    }
    Ok(())
}

#[sysfail(log(level = "error"))]
pub fn handle_deflect_events(
    mut deflect_events: EventReader<DeflectedByEnemyEvent>,
    mut enemies: Query<(&mut Enemy, &mut Constitution, &AnimationEntityLink)>,
    mut animation_players: Query<&mut AnimationPlayer>,
) -> Result<()> {
    for attack in deflect_events.iter() {
        for (mut enemy, mut constitution, animation_entity_link) in enemies.iter_mut() {
            constitution.take_posture_damage(attack); // Enemies don't get any bonus on deflections
            enemy.block();
            let mut animation_player = animation_players.get_mut(**animation_entity_link)?;
            // Force the animation to restart in case we are already blocking the previous attack
            animation_player.pause();
        }
    }
    Ok(())
}