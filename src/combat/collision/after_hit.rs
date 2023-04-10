use crate::combat::collision::{BlockedByEnemyEvent, DeflectedByEnemyEvent, EnemyHurtEvent};
use crate::combat::{Constitution, Enemy, EnemyCombatState};
use crate::level_instantiation::spawning::AnimationEntityLink;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;

pub(crate) fn handle_hurt_events(
    mut hurt_events: EventReader<EnemyHurtEvent>,
    mut enemies: Query<(&mut Enemy, &EnemyCombatState, &mut Constitution)>,
) {
    for attack in hurt_events.iter() {
        for (mut enemy, combat_state, mut constitution) in enemies.iter_mut() {
            constitution.take_full_damage(attack);

            match combat_state {
                EnemyCombatState::Deathblow => constitution.die(),
                EnemyCombatState::HyperArmor => {}
                _ => enemy.hurt(),
            }

            enemy.time_since_hurt_or_block = 0.0;
        }
    }
}

#[sysfail(log(level = "error"))]
pub(crate) fn handle_block_events(
    mut block_events: EventReader<BlockedByEnemyEvent>,
    mut enemies: Query<(&mut Enemy, &mut Constitution, &AnimationEntityLink)>,
    mut animation_players: Query<&mut AnimationPlayer>,
) -> Result<()> {
    for attack in block_events.iter() {
        for (mut enemy, mut constitution, animation_entity_link) in enemies.iter_mut() {
            constitution.take_posture_damage(attack);
            enemy.block();
            let mut animation_player = animation_players
                .get_mut(**animation_entity_link)
                .context("Animation player link held invalid reference")?;
            // Force the animation to restart in case we are already blocking the previous attack
            animation_player.pause();
            enemy.time_since_hurt_or_block = 0.0;
        }
    }
    Ok(())
}

#[sysfail(log(level = "error"))]
pub(crate) fn handle_deflect_events(
    mut deflect_events: EventReader<DeflectedByEnemyEvent>,
    mut enemies: Query<(&mut Enemy, &mut Constitution, &AnimationEntityLink)>,
    mut animation_players: Query<&mut AnimationPlayer>,
) -> Result<()> {
    for attack in deflect_events.iter() {
        for (mut enemy, mut constitution, animation_entity_link) in enemies.iter_mut() {
            constitution.take_posture_damage(attack); // Enemies don't get any bonus on deflections
            enemy.block();
            let mut animation_player = animation_players
                .get_mut(**animation_entity_link)
                .context("Animation player link held invalid entity")?;
            // Force the animation to restart in case we are already blocking the previous attack
            animation_player.pause();
        }
    }
    Ok(())
}
