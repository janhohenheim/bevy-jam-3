use crate::combat::ConditionTracker;
use crate::movement::general_movement::{Grounded, Height};
use crate::player_control::player_embodiment::Player;
use anyhow::Result;
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;
use bevy_rapier3d::prelude::*;
use oxidized_navigation::query::{find_path, perform_string_pulling_on_path};
use oxidized_navigation::{NavMesh, NavMeshSettings};

#[sysfail(log(level = "error"))]
pub(crate) fn update_condition_tracker(
    mut combatants: Query<
        (
            Entity,
            &mut ConditionTracker,
            &Transform,
            &Height,
            &Grounded,
            &Collider,
        ),
        Without<Player>,
    >,
    player: Query<(Entity, &Transform, &Height), With<Player>>,
    rapier_context: Res<RapierContext>,
    nav_mesh_settings: Res<NavMeshSettings>,
    nav_mesh: Res<NavMesh>,
) -> Result<()> {
    for (
        combatant_entity,
        mut condition_tracker,
        combatant_transform,
        combatant_height,
        combatant_grounded,
        collider,
    ) in combatants.iter_mut()
    {
        condition_tracker.grounded = combatant_grounded.0;
        for (player_entity, player_transform, player_height) in player.iter() {
            let from = combatant_transform.translation;
            let to = player_transform.translation
                + Vec3::Y * (combatant_height.half() - player_height.half());
            let has_line_of_sight = get_line_of_sight(
                &rapier_context,
                from,
                combatant_transform.rotation,
                collider,
                combatant_entity,
                to,
                player_entity,
            );
            condition_tracker.player_direction = to - from;

            // if the navmesh is not loaded, we might as well pretend we have line of sight.
            // Otherwise, this gets overwritten below.
            condition_tracker.has_line_of_sight = true;
            condition_tracker.line_of_sight_direction = condition_tracker.player_direction;
            if !has_line_of_sight {
                if let Ok(nav_mesh) = nav_mesh.get().read() {
                    if let Ok(path) = find_path(&nav_mesh, &nav_mesh_settings, from, to, None, None)
                    {
                        condition_tracker.has_line_of_sight = false;
                        let to_origin = Vec3::Y * combatant_height.half();
                        let path: Vec<_> =
                            perform_string_pulling_on_path(&nav_mesh, from, to, &path)
                                .map_err(|e| anyhow::Error::msg(format!("{e:?}")))?
                                .into_iter()
                                .skip(1) // to ground
                                .map(|location| location - from + to_origin)
                                .filter(|dir| dir.length_squared() > 0.1)
                                .take(2)
                                .collect();

                        // Expect 2: next node and from ground to player
                        if path.len() >= 2 {
                            condition_tracker.line_of_sight_direction = path[0];
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn get_line_of_sight(
    rapier_context: &RapierContext,
    origin: Vec3,
    rotation: Quat,
    shape: &Collider,
    origin_entity: Entity,
    target: Vec3,
    target_entity: Entity,
) -> bool {
    const MAX_TOI: f32 = 40.0;
    let filter = QueryFilter::new()
        .exclude_collider(origin_entity)
        .exclude_sensors();

    rapier_context
        .cast_shape(origin, rotation, target - origin, shape, MAX_TOI, filter)
        .and_then(|(entity, toi)| (entity == target_entity).then_some(toi))
        .is_some()
}
