use crate::combat::collision::PlayerHitEvent;
use crate::combat::Attack;
use crate::player_control::player_embodiment::combat::{PlayerCombatKind, PlayerCombatState};
use crate::player_control::player_embodiment::Player;
use anyhow::Result;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;
use serde::{Deserialize, Serialize};

#[sysfail(log(level = "error"))]
pub fn handle_player_being_hit(
    mut hit_events: EventReader<PlayerHitEvent>,
    mut players: Query<(&Transform, &PlayerCombatState), With<Player>>,
    mut hurt_events: EventWriter<PlayerHurtEvent>,
    mut block_events: EventWriter<BlockedByPlayerEvent>,
    mut deflect_events: EventWriter<DeflectedByPlayerEvent>,
) -> Result<()> {
    for event in hit_events.iter() {
        for (transform, combat_state) in players.iter_mut() {
            if combat_state.kind != PlayerCombatKind::Block {
                hurt_events.send(event.into());
            } else {
                let angle = transform
                    .forward()
                    .xz()
                    .angle_between(event.target_to_contact.xz())
                    .to_degrees();
                if angle.abs() > 100.0 {
                    hurt_events.send(event.into());
                } else if combat_state.time_in_state < 0.2 {
                    // TODO: scale with repeated deflects
                    deflect_events.send(event.into())
                } else {
                    block_events.send(event.into());
                }
            }
        }
    }
    Ok(())
}

#[derive(
    Debug, Clone, PartialEq, Reflect, Serialize, Deserialize, FromReflect, Default, Deref, DerefMut,
)]
#[reflect(Serialize, Deserialize)]
pub struct PlayerHurtEvent(pub Attack);

impl From<&PlayerHitEvent> for PlayerHurtEvent {
    fn from(event: &PlayerHitEvent) -> Self {
        Self(event.attack.clone())
    }
}

#[derive(
    Debug, Clone, PartialEq, Reflect, Serialize, Deserialize, FromReflect, Default, Deref, DerefMut,
)]
#[reflect(Serialize, Deserialize)]
pub struct BlockedByPlayerEvent(pub Attack);

impl From<&PlayerHitEvent> for BlockedByPlayerEvent {
    fn from(event: &PlayerHitEvent) -> Self {
        Self(event.attack.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize, FromReflect)]
#[reflect(Serialize, Deserialize)]
pub struct DeflectedByPlayerEvent {
    pub attack: Attack,
    pub attacker: Entity,
}

impl From<&PlayerHitEvent> for DeflectedByPlayerEvent {
    fn from(event: &PlayerHitEvent) -> Self {
        Self {
            attack: event.attack.clone(),
            attacker: event.source,
        }
    }
}
