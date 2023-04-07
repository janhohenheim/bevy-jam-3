use crate::combat::AttackHitbox;
use crate::player_control::player_embodiment::Player;
use crate::world_interaction::interactions_ui::unpack_event;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn on_hit(
    mut collision_events: EventReader<CollisionEvent>,
    players: Query<(), With<Player>>,
    attacks: Query<(&AttackHitbox,)>,
) {
    for event in collision_events.iter() {
        let (entity_a, entity_b, ongoing) = unpack_event(event);
        let Some((_player,_hitbox)) = determine_player_and_target(&players, &attacks, entity_a, entity_b) else {
            continue;
        };
        info!("Player hit by attack: {ongoing}");
    }
}

fn determine_player_and_target(
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
