use crate::combat::{ForceFn, ForceFnInput, ForceFnOutput};
use crate::util::smoothness_to_lerp_factor;
use crate::util::trait_extension::Vec3Ext;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn accelerate_towards_player(acceleration: f32) -> Box<dyn ForceFn> {
    Box::new(
        move |ForceFnInput {
                  transform,
                  line_of_sight_path,
                  mass,
                  velocity,
                  config,
                  dt,
                  ..
              }: ForceFnInput| {
            let direction = (line_of_sight_path[0] - transform.translation).normalize();
            let force = direction * acceleration * mass;

            let up = transform.up();
            let horizontal_movement = velocity.split(up).horizontal;
            let rotation = (!horizontal_movement.is_approx_zero()).then(|| {
                let target_rotation = transform.looking_to(horizontal_movement, up).rotation;
                let smoothness = config.characters.rotation_smoothing;
                let factor = smoothness_to_lerp_factor(smoothness, dt);
                transform.rotation.slerp(target_rotation, factor)
            });
            ForceFnOutput {
                force: ExternalForce { force, ..default() },
                rotation,
                ..default()
            }
        },
    )
}
