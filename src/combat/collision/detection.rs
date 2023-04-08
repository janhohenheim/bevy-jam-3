use crate::combat::{Attack, AttackHitbox, Combatant};
use crate::player_control::player_embodiment::Player;
use crate::world_interaction::interactions_ui::unpack_event;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_mod_sysfail::sysfail;
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize, FromReflect)]
#[reflect(Serialize, Deserialize)]
pub struct PlayerHitEvent {
    pub(crate) attack: Attack,
    pub(crate) normal: Vec3,
}

#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize, FromReflect)]
#[reflect(Serialize, Deserialize)]
pub struct EnemyHitEvent {
    pub(crate) target: Entity,
    pub(crate) attack: Attack,
    pub(crate) normal: Vec3,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Resource,
    Reflect,
    Serialize,
    Deserialize,
    FromReflect,
    Default,
    Deref,
    DerefMut,
)]
#[reflect(Serialize, Resource, Deserialize)]
/// Necessary because our collision events are not using sensors because we need the manifold normal.
/// Sensors only report intersections, not locations, so we use non-sensors without any solvers, which means no displacement takes place.
/// This however means that continuous penetrations are reported repeatedly, so we need to track which ones we already handled.
pub struct HitCache(HashMap<Entity, HitboxHits>);

#[derive(
    Debug, Clone, Reflect, Serialize, Deserialize, FromReflect, Default, PartialEq, Eq, Hash,
)]
#[reflect(Serialize, Deserialize)]
pub struct HitboxHits {
    pub attack_name: String,
    pub targets: Vec<Entity>,
}

#[derive(Debug, Clone)]
pub struct Hit {
    hitbox: Entity,
    target: Entity,
    attack: Attack,
}

impl HitCache {
    pub fn update_and_check_contained(
        &mut self,
        Hit {
            hitbox,
            target,
            attack,
        }: Hit,
    ) -> bool {
        if let Some(hits) = self.get_mut(&hitbox) {
            if hits.attack_name == attack.name {
                if hits.targets.contains(&target) {
                    true
                } else {
                    hits.targets.push(target);
                    false
                }
            } else {
                // Hitbox has changed attack
                self.insert(
                    hitbox,
                    HitboxHits {
                        attack_name: attack.name.clone(),
                        targets: vec![target],
                    },
                );
                false
            }
        } else {
            self.insert(
                hitbox,
                HitboxHits {
                    attack_name: attack.name.clone(),
                    targets: vec![target],
                },
            );
            false
        }
    }

    pub fn remove_expired(&mut self, hitboxes: &Query<&AttackHitbox>) {
        self.retain(|hitbox, hit| {
            hitboxes
                .get(*hitbox)
                .map(|hitbox| hitbox.active && hitbox.attack.name == hit.attack_name)
                .unwrap_or_default()
        });
    }
}

#[sysfail(log(level = "error"))]
pub fn detect_hits(
    mut collision_events: EventReader<CollisionEvent>,
    players: Query<(), With<Player>>,
    combatants: Query<(), With<Combatant>>,
    attacks: Query<(&AttackHitbox,)>,
    mut player_hit_events: EventWriter<PlayerHitEvent>,
    mut enemy_hit_events: EventWriter<EnemyHitEvent>,
    rapier_context: Res<RapierContext>,
    mut hit_cache: ResMut<HitCache>,
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

        if let Some((player_entity, hitbox_entity)) =
            determine_player_and_hitbox(&players, &attacks, entity_a, entity_b)
        {
            if let Some(hitbox) = get_active_hitbox(hitbox_entity)? {
                if hitbox.active
                    && !hit_cache.update_and_check_contained(Hit {
                        target: player_entity,
                        hitbox: hitbox_entity,
                        attack: hitbox.attack.clone(),
                    })
                {
                    let normal = get_contact_normal(player_entity, hitbox_entity, &rapier_context)?;
                    player_hit_events.send(PlayerHitEvent {
                        attack: hitbox.attack.clone(),
                        normal,
                    });
                }
            }
        } else if let Some((enemy_entity, hitbox_entity)) =
            determine_enemy_and_hitbox(&combatants, &attacks, entity_a, entity_b)
        {
            if let Some(hitbox) = get_active_hitbox(hitbox_entity)? {
                if hitbox.active
                    && !hit_cache.update_and_check_contained(Hit {
                        target: enemy_entity,
                        hitbox: hitbox_entity,
                        attack: hitbox.attack.clone(),
                    })
                {
                    let normal = get_contact_normal(enemy_entity, hitbox_entity, &rapier_context)?;
                    enemy_hit_events.send(EnemyHitEvent {
                        target: enemy_entity,
                        attack: hitbox.attack.clone(),
                        normal,
                    });
                }
            }
        }
    }
    Ok(())
}

pub fn clear_cache(mut hit_cache: ResMut<HitCache>, attacks: Query<&AttackHitbox>) {
    hit_cache.remove_expired(&attacks);
}

fn get_contact_normal(
    target: Entity,
    hitbox: Entity,
    rapier_context: &RapierContext,
) -> Result<Vec3> {
    let contact_pair = rapier_context
        .contact_pair(target, hitbox)
        .context("Failed to get contact pair")?;
    let (manifold, _contact) = contact_pair
        .find_deepest_contact()
        .context("Failed to find deepest contact")?;
    Ok(manifold.normal())
}

fn get_target_to_hitbox(
    target: Entity,
    hitbox: Entity,
    transforms: &Query<&Transform>,
) -> Result<Vec3> {
    let target_origin = transforms.get(target)?;
    let hitbox_origin = transforms.get(hitbox)?;
    let target_to_hitbox = hitbox_origin.translation - target_origin.translation;
    Ok(target_to_hitbox)
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
