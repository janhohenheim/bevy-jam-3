use crate::combat::collision::detection::EnemyHitEvent;
use crate::combat::{Attack, Enemy, EnemyCombatState};
use anyhow::Result;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;
use serde::{Deserialize, Serialize};

#[sysfail(log(level = "error"))]
pub(crate) fn handle_enemy_being_hit(
    mut hit_events: EventReader<EnemyHitEvent>,
    mut combatants: Query<(&Enemy, &Transform)>,
    mut hurt_events: EventWriter<EnemyHurtEvent>,
    mut block_events: EventWriter<BlockedByEnemyEvent>,
    mut deflect_events: EventWriter<DeflectedByEnemyEvent>,
) -> Result<()> {
    for event in hit_events.iter() {
        let (combatant, transform) = combatants
            .get_mut(event.target)
            .expect("Failed to get combatant from hit event");

        let angle = transform
            .forward()
            .xz()
            .angle_between(event.target_to_contact.xz())
            .to_degrees();
        match combatant.current_move() {
            Some(move_) => match move_.metadata.state {
                EnemyCombatState::Deathblow => {
                    hurt_events.send(event.into());
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

#[derive(
    Debug, Clone, PartialEq, Reflect, Serialize, Deserialize, FromReflect, Default, Deref, DerefMut,
)]
#[reflect(Serialize, Deserialize)]
pub(crate) struct EnemyHurtEvent(pub(crate) Attack);

impl From<&EnemyHitEvent> for EnemyHurtEvent {
    fn from(event: &EnemyHitEvent) -> Self {
        Self(event.attack.clone())
    }
}

#[derive(
    Debug, Clone, PartialEq, Reflect, Serialize, Deserialize, FromReflect, Default, Deref, DerefMut,
)]
#[reflect(Serialize, Deserialize)]
pub(crate) struct BlockedByEnemyEvent(pub(crate) Attack);

impl From<&EnemyHitEvent> for BlockedByEnemyEvent {
    fn from(event: &EnemyHitEvent) -> Self {
        Self(event.attack.clone())
    }
}

#[derive(
    Debug, Clone, PartialEq, Reflect, Serialize, Deserialize, FromReflect, Default, Deref, DerefMut,
)]
#[reflect(Serialize, Deserialize)]
pub(crate) struct DeflectedByEnemyEvent(pub(crate) Attack);

impl From<&EnemyHitEvent> for DeflectedByEnemyEvent {
    fn from(event: &EnemyHitEvent) -> Self {
        Self(event.attack.clone())
    }
}
