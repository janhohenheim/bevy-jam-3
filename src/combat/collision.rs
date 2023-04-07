use crate::combat::{Attack, AttackHitbox};
use crate::player_control::player_embodiment::Player;
use crate::world_interaction::interactions_ui::unpack_event;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_mod_sysfail::sysfail;
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize, FromReflect)]
#[reflect(Serialize, Deserialize)]
pub struct HitEvent {
    attack: Attack,
}

#[sysfail(log(level = "error"))]
pub fn on_hit(
    mut collision_events: EventReader<CollisionEvent>,
    players: Query<(), With<Player>>,
    attacks: Query<(&AttackHitbox,)>,
) -> Result<()> {
    for event in collision_events.iter() {
        let (entity_a, entity_b, ongoing) = unpack_event(event);
        if !ongoing {
            continue;
        }
        let Some((_player,hitbox)) = determine_player_and_hitbox(&players, &attacks, entity_a, entity_b) else {
            continue;
        };
        let (hitbox,) = attacks
            .get(hitbox)
            .context("Failed to get attack data from colliding entity")?;
        if hitbox.active {}
    }
    Ok(())
}

fn determine_player_and_hitbox(
    players: &Query<(), With<Player>>,
    attacks: &Query<(&AttackHitbox,)>,
    entity_a: Entity,
    entity_b: Entity,
) -> Option<(Entity, Entity)> {
    if players.get(entity_a).is_ok() && attacks.get(entity_b).is_ok() {
        Some((entity_a, entity_b))
    } else if players.get(entity_b).is_ok() && attacks.get(entity_a).is_ok() {
        Some((entity_b, entity_a))
    } else {
        None
    }
}
