use crate::combat::ConditionTracker;
use crate::movement::general_movement::Height;
use crate::player_control::player_embodiment::Player;
use anyhow::Result;
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;
use bevy_rapier3d::prelude::*;
use oxidized_navigation::query::{find_path, perform_string_pulling_on_path};
use oxidized_navigation::{NavMesh, NavMeshSettings};

#[sysfail(log(level = "error"))]
pub fn update_condition_tracker(
    mut combatants: Query<(Entity, &mut ConditionTracker, &Transform, &Height), Without<Player>>,
    player: Query<(Entity, &Transform, &Height), With<Player>>,
    rapier_context: Res<RapierContext>,
    nav_mesh_settings: Res<NavMeshSettings>,
    nav_mesh: Res<NavMesh>,
) -> Result<()> {
    for (combatant_entity, mut condition_tracker, combatant_transform, combatant_height) in
        combatants.iter_mut()
    {
        for (player_entity, player_transform, player_height) in player.iter() {
            let from = combatant_transform.translation;
            let to = player_transform.translation
                + Vec3::Y * (combatant_height.half() - player_height.half());
            let line_of_sight =
                get_line_of_sight(&rapier_context, from, combatant_entity, to, player_entity);
            condition_tracker.active = true;
            condition_tracker.player_direction = to - from;

            if let Some(_line_of_sight) = line_of_sight {
                condition_tracker.has_line_of_sight = true;
                condition_tracker.line_of_sight_direction = condition_tracker.player_direction;
            } else {
                condition_tracker.has_line_of_sight = false;
                if let Ok(nav_mesh) = nav_mesh.get().read() {
                    let to_origin = Vec3::Y * combatant_height.half();
                    if let Ok(path) = find_path(&nav_mesh, &nav_mesh_settings, from, to, None, None)
                    {
                        let mut path: Vec<_> =
                            perform_string_pulling_on_path(&nav_mesh, from, to, &path)
                                .map_err(|e| anyhow::Error::msg(format!("{e:?}")))?
                                .into_iter()
                                .filter(|location| (*location - from).length_squared() > 0.1)
                                .map(|location| location + to_origin)
                                .skip(1) // to ground
                                .collect();
                        path.remove(path.len() - 1); // off from ground to player
                        condition_tracker.line_of_sight_direction = if path.is_empty() {
                            condition_tracker.player_direction
                        } else {
                            path[0]
                        };
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
        .and_then(|(entity, toi)| (entity == target_entity).then_some(toi))
}
