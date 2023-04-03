use crate::combat::ConditionTracker;
use crate::player_control::player_embodiment::Player;
use anyhow::Result;
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;
use bevy_rapier3d::prelude::*;
use oxidized_navigation::query::{find_path, perform_string_pulling_on_path};
use oxidized_navigation::{NavMesh, NavMeshSettings};

#[sysfail(log(level = "error"))]
pub fn update_condition_tracker(
    mut combatants: Query<(Entity, &mut ConditionTracker, &Transform), Without<Player>>,
    player: Query<(Entity, &Transform), With<Player>>,
    rapier_context: Res<RapierContext>,
    nav_mesh_settings: Res<NavMeshSettings>,
    nav_mesh: Res<NavMesh>,
) -> Result<()> {
    for (combatant_entity, mut condition_tracker, combatant_transform) in combatants.iter_mut() {
        for (player_entity, player_transform) in player.iter() {
            let from = combatant_transform.translation;
            let to = player_transform.translation;
            let line_of_sight =
                get_line_of_sight(&rapier_context, from, combatant_entity, to, player_entity);
            condition_tracker.active = true;
            condition_tracker.player_direction = to - from;

            if let Some(_line_of_sight) = line_of_sight {
                condition_tracker.has_line_of_sight = true;
                condition_tracker.line_of_sight_path = vec![to];
            } else {
                condition_tracker.has_line_of_sight = false;
                if let Ok(nav_mesh) = nav_mesh.get().read() {
                    if let Ok(path) = find_path(&nav_mesh, &nav_mesh_settings, from, to, None, None)
                    {
                        let path = perform_string_pulling_on_path(&nav_mesh, from, to, &path)
                            .map_err(|e| anyhow::Error::msg(format!("{e:?}")))?;
                        condition_tracker.line_of_sight_path = path;
                    } else {
                        condition_tracker.active = false;
                    }
                } else {
                    condition_tracker.active = false;
                }
            };
        }
    }
    Ok(())
}

fn get_line_of_sight(
    rapier_context: &RapierContext,
    origin: Vec3,
    origin_entity: Entity,
    target: Vec3,
    target_entity: Entity,
) -> Option<f32> {
    const MAX_TOI: f32 = 10.0;
    const SOLID: bool = true;
    let filter = QueryFilter::new()
        .exclude_collider(origin_entity)
        .exclude_sensors();

    rapier_context
        .cast_ray(origin, target - origin, MAX_TOI, SOLID, filter)
        .and_then(|(entity, toi)| (entity == target_entity).then(|| toi))
}
