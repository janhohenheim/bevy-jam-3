use crate::combat::{Attack, AttackHitbox, Combatant};
use crate::player_control::player_embodiment::Player;
use crate::world_interaction::interactions_ui::unpack_event;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_mod_sysfail::sysfail;
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize, FromReflect)]
#[reflect(Serialize, Deserialize)]
pub struct PlayerHitEvent {
    pub(crate) attack: Attack,
}

#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize, FromReflect)]
#[reflect(Serialize, Deserialize)]
pub struct EnemyHitEvent {
    pub(crate) target: Entity,
    pub(crate) attack: Attack,
}

#[sysfail(log(level = "error"))]
pub fn detect_hits(
    mut collision_events: EventReader<CollisionEvent>,
    players: Query<(), With<Player>>,
    combatants: Query<(), With<Combatant>>,
    attacks: Query<(&AttackHitbox,)>,
    mut player_hit_events: EventWriter<PlayerHitEvent>,
    mut enemy_hit_events: EventWriter<EnemyHitEvent>,
) -> Result<()> {
    let get_active_hitbox = |entity: Entity| -> Result<Option<&AttackHitbox>> {
        let (hitbox,) = attacks
            .get(entity)
            .ok()
            .context("Failed to get attack data from colliding entity")?;
        let result = if hitbox.active { Some(hitbox) } else { None };
        Ok(result)
    };
    for event in collision_events.iter() {
        let (entity_a, entity_b, ongoing) = unpack_event(event);
        if !ongoing {
            continue;
        }
        if let Some((_player, hitbox)) =
            determine_player_and_hitbox(&players, &attacks, entity_a, entity_b)
        {
            if let Some(hitbox) = get_active_hitbox(hitbox)? {
                player_hit_events.send(PlayerHitEvent {
                    attack: hitbox.attack,
                });
            }
        } else if let Some((enemy, hitbox)) =
            determine_enemy_and_hitbox(&combatants, &attacks, entity_a, entity_b)
        {
            if let Some(hitbox) = get_active_hitbox(hitbox)? {
                enemy_hit_events.send(EnemyHitEvent {
                    target: enemy,
                    attack: hitbox.attack,
                });
            }
        }
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

fn determine_enemy_and_hitbox(
    combatants: &Query<(), With<Combatant>>,
    attacks: &Query<(&AttackHitbox,)>,
    entity_a: Entity,
    entity_b: Entity,
) -> Option<(Entity, Entity)> {
    if combatants.get(entity_a).is_ok() && attacks.get(entity_b).is_ok() {
        Some((entity_a, entity_b))
    } else if combatants.get(entity_b).is_ok() && attacks.get(entity_a).is_ok() {
        Some((entity_b, entity_a))
    } else {
        None
    }
}
