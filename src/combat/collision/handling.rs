use crate::combat::collision::detection::EnemyHitEvent;
use crate::combat::{Attack, Enemy, EnemyCombatState};
use crate::world_interaction::side_effects::{SideEffect, SideEffects};
use anyhow::Result;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;

#[sysfail(log(level = "error"))]
pub(crate) fn handle_enemy_being_hit(
    mut hit_events: EventReader<EnemyHitEvent>,
    mut enemies: Query<(&mut Enemy, &Transform)>,
    mut hurt_events: EventWriter<EnemyHurtEvent>,
    mut block_events: EventWriter<BlockedByEnemyEvent>,
    mut deflect_events: EventWriter<DeflectedByEnemyEvent>,
    side_effects: Res<SideEffects>,
) -> Result<()> {
    for event in hit_events.iter() {
        let health_side_effect = side_effects.get_factored(SideEffect::HealthDamage, 0.15);
        let posture_side_effect = side_effects.get_factored(SideEffect::AttackPostureDamage, 0.1);
        let knockback_side_effect = side_effects.get_factored(SideEffect::AttackKnockback, 0.2);
        let event = EnemyHitEvent {
            attack: Attack {
                name: event.attack.name.clone(),
                health_damage: event.attack.health_damage * health_side_effect,
                posture_damage: event.attack.posture_damage * posture_side_effect,
                knockback: event.attack.knockback * knockback_side_effect,
            },
            ..event.clone()
        };
        let (mut enemy, transform) = enemies
            .get_mut(event.target)
            .expect("Failed to get combatant from hit event");

        let angle = transform
            .forward()
            .xz()
            .angle_between(event.target_to_contact.xz())
            .to_degrees();
        match enemy.current_move() {
            Some(move_) => match move_.metadata.state {
                EnemyCombatState::Deathblow => {
                    //hurt_events.send(event.into());
                    enemy.die();
                }
                EnemyCombatState::Vulnerable => {
                    hurt_events.send(event.into());
                }
                EnemyCombatState::OnGuard => {
                    if angle < get_max_block_angle() {
                        if roll_for_deflect() {
                            deflect_events.send(event.into());
                        } else {
                            block_events.send(event.into());
                        }
                    } else {
                        hurt_events.send(event.into());
                    }
                }
                EnemyCombatState::HyperArmor => {
                    hurt_events.send(event.into());
                }
                EnemyCombatState::Dying => {}
            },
            None => {}
        }
    }
    Ok(())
}

fn get_max_block_angle() -> f32 {
    100.0
}

fn roll_for_deflect() -> bool {
    const CHANCE_FOR_DEFLECT: f32 = 1.0 / 3.0;
    let rand = rand::random::<f32>();
    rand < CHANCE_FOR_DEFLECT
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct EnemyHurtEvent {
    pub(crate) enemy: Entity,
    pub(crate) attack: Attack,
}

impl From<EnemyHitEvent> for EnemyHurtEvent {
    fn from(event: EnemyHitEvent) -> Self {
        Self {
            enemy: event.target,
            attack: event.attack.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BlockedByEnemyEvent {
    pub(crate) enemy: Entity,
    pub(crate) attack: Attack,
}

impl From<EnemyHitEvent> for BlockedByEnemyEvent {
    fn from(event: EnemyHitEvent) -> Self {
        Self {
            enemy: event.target,
            attack: event.attack.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct DeflectedByEnemyEvent {
    pub(crate) enemy: Entity,
    pub(crate) attack: Attack,
}

impl From<EnemyHitEvent> for DeflectedByEnemyEvent {
    fn from(event: EnemyHitEvent) -> Self {
        Self {
            enemy: event.target,
            attack: event.attack.clone(),
        }
    }
}
