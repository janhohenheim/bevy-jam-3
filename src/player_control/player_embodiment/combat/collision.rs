use crate::combat::collision::PlayerHitEvent;
use crate::combat::Attack;
use crate::player_control::player_embodiment::combat::{
    BlockHistory, PlayerCombatKind, PlayerCombatState,
};
use crate::player_control::player_embodiment::Player;
use anyhow::Result;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;
use serde::{Deserialize, Serialize};

#[sysfail(log(level = "error"))]
pub fn handle_player_being_hit(
    mut hit_events: EventReader<PlayerHitEvent>,
    mut players: Query<(&Transform, &mut PlayerCombatState, &mut BlockHistory), With<Player>>,
    mut hurt_events: EventWriter<PlayerHurtEvent>,
    mut block_events: EventWriter<BlockedByPlayerEvent>,
    mut deflect_events: EventWriter<DeflectedByPlayerEvent>,
) -> Result<()> {
    for event in hit_events.iter() {
        for (transform, mut combat_state, mut block_history) in players.iter_mut() {
            if combat_state.kind != PlayerCombatKind::Block {
                hurt_events.send(event.into());
            } else {
                let angle = transform
                    .forward()
                    .xz()
                    .angle_between(event.target_to_contact.xz())
                    .to_degrees();
                if angle.abs() > get_max_block_angle() {
                    hurt_events.send(event.into());
                    combat_state.time_since_hurt_or_block = 0.0;
                } else if combat_state.time_in_state < get_max_deflect_time(&block_history) {
                    deflect_events.send(event.into());
                    block_history.mark_last_as_deflect();
                } else {
                    block_events.send(event.into());
                    combat_state.time_since_hurt_or_block = 0.0;
                }
            }
        }
    }
    Ok(())
}

fn get_max_block_angle() -> f32 {
    100.0
}

fn get_max_deflect_time(block_history: &BlockHistory) -> f32 {
    // Adapted from: <https://www.youtube.com/watch?v=GRdHVXfVbfI>
    let base_max_deflect_time = 0.3;
    // Using saturating_sub because there might be 0 blocks younger than the specified time if we've been holding block for a while.
    let blocks_before_current = block_history.count_younger_than(0.5).saturating_sub(1);
    let factor = (1.0 - 0.25 * blocks_before_current as f32).max(0.0);
    base_max_deflect_time * factor
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
