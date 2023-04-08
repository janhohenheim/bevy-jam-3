use crate::combat::{Attack, AttackHitbox, Combatant};
use crate::player_control::player_embodiment::Player;
use crate::world_interaction::interactions_ui::unpack_event;
use anyhow::{Context, Error, Result};
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
    Debug, Clone, PartialEq, Resource, Reflect, Serialize, Deserialize, FromReflect, Default,
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
    pub fn contains(
        &self,
        Hit {
            hitbox,
            target,
            attack,
        }: &Hit,
    ) -> bool {
        self.0
            .get(hitbox)
            .map(|hits| hits.attack_name == attack.name && hits.targets.contains(target))
            .unwrap_or_default()
    }

    pub fn insert(
        &mut self,
        Hit {
            hitbox,
            target,
            attack,
        }: Hit,
    ) {
        if let Some(hits) = self.0.get_mut(&hitbox) {
            if hits.attack_name == attack.name {
                if !hits.targets.contains(&target) {
                    hits.targets.push(target);
                }
            } else {
                // Hitbox has changed attack
                self.0.insert(
                    hitbox,
                    HitboxHits {
                        attack_name: attack.name.clone(),
                        targets: vec![target],
                    },
                );
            }
        } else {
            self.0.insert(
                hitbox,
                HitboxHits {
                    attack_name: attack.name.clone(),
                    targets: vec![target],
                },
            );
        }
    }

    pub fn remove_expired(&mut self, hitboxes: &Query<&AttackHitbox>) {
        self.0.retain(|hitbox, hit| {
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
    for event in collision_events.iter() {
        let (entity_a, entity_b, ongoing) = unpack_event(event);
        if !ongoing {
            continue;
        }

        let mut handle_potential_hit =
            |target_entity: Entity,
             hitbox_entity: Entity,
             mut send_fn: Box<dyn FnMut(Entity, &AttackHitbox, Vec3)>|
             -> Result<()> {
                if let Some(hitbox) = get_active_hitbox(&attacks, hitbox_entity)? {
                    let hit = Hit {
                        target: target_entity,
                        hitbox: hitbox_entity,
                        attack: hitbox.attack.clone(),
                    };
                    if hitbox.active && !hit_cache.contains(&hit) {
                        if let Some(normal) =
                            get_contact_normal(target_entity, hitbox_entity, &rapier_context)
                        {
                            hit_cache.insert(hit);
                            send_fn(target_entity, hitbox, normal);
                        }
                    }
                }
                Ok(())
            };

        if let Some((player_entity, hitbox_entity)) =
            determine_player_and_hitbox(&players, &attacks, entity_a, entity_b)
        {
            let send_player_hit = |_target_entity: Entity, hitbox: &AttackHitbox, normal: Vec3| {
                player_hit_events.send(PlayerHitEvent {
                    attack: hitbox.attack.clone(),
                    normal,
                });
            };

            handle_potential_hit(player_entity, hitbox_entity, Box::new(send_player_hit))?;
        } else if let Some((enemy_entity, hitbox_entity)) =
            determine_enemy_and_hitbox(&combatants, &attacks, entity_a, entity_b)
        {
            let send_enemy_hit = |target_entity: Entity, hitbox: &AttackHitbox, normal: Vec3| {
                enemy_hit_events.send(EnemyHitEvent {
                    target: target_entity,
                    attack: hitbox.attack.clone(),
                    normal,
                });
            };

            handle_potential_hit(enemy_entity, hitbox_entity, Box::new(send_enemy_hit))?;
        }
    }
    Ok(())
}

fn get_active_hitbox<'a>(
    attacks: &'a Query<(&AttackHitbox,)>,
    entity: Entity,
) -> Result<Option<&'a AttackHitbox>, Error> {
    let (hitbox,) = attacks
        .get(entity)
        .ok()
        .context("Failed to get attack data from colliding entity")?;
    let result = if hitbox.active { Some(hitbox) } else { None };
    Ok(result)
}

pub fn clear_cache(mut hit_cache: ResMut<HitCache>, attacks: Query<&AttackHitbox>) {
    hit_cache.remove_expired(&attacks);
}

fn get_contact_normal(
    target: Entity,
    hitbox: Entity,
    rapier_context: &RapierContext,
) -> Option<Vec3> {
    let contact_pair = rapier_context.contact_pair(target, hitbox)?;
    if !contact_pair.has_any_active_contacts() {
        return None;
    }
    // Only one manifold because we are dealing with convex primitive shapes only
    assert_eq!(
        contact_pair.manifolds_len(),
        1,
        "Expected one manifold since we are dealing with convex shapes only."
    );
    let manifold = contact_pair.manifold(0).unwrap();
    info!("Local-space contact normal: {}", manifold.local_n1());
    info!("Local-space contact normal: {}", manifold.local_n2());
    info!("World-space contact normal: {}", manifold.normal());

    for contact in manifold.solver_contacts() {
        info!("Solver contact point: {}", contact.point());
        info!("Solver contact dist: {}", contact.dist());
    }
    Some(manifold.normal())
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
