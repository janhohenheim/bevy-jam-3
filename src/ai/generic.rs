use crate::combat::{ForceFn, ForceFnInput, ForceFnOutput};
use crate::file_system_interaction::config::GameConfig;
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

            let rotation = rotation_to_horizontal(transform, velocity, config, dt);
            ForceFnOutput {
                force: ExternalForce { force, ..default() },
                rotation,
                ..default()
            }
        },
    )
}

pub fn face_player() -> Box<dyn ForceFn> {
    Box::new(
        move |ForceFnInput {
                  transform,
                  player_direction,
                  config,
                  dt,
                  ..
              }: ForceFnInput| {
            let rotation = rotation_to_horizontal(transform, player_direction, config, dt);
            ForceFnOutput {
                rotation,
                ..default()
            }
        },
    )
}

fn rotation_to_horizontal(
    transform: Transform,
    direction: Vec3,
    config: GameConfig,
    dt: f32,
) -> Option<Quat> {
    let up = transform.up();
    let horizontal_direction = direction.split(up).horizontal;
    if horizontal_direction.is_approx_zero() {
        return None;
    }

    let target_rotation = transform.looking_to(horizontal_direction, up).rotation;
    let smoothness = config.characters.rotation_smoothing;
    let factor = smoothness_to_lerp_factor(smoothness, dt);
    let rotation = transform.rotation.slerp(target_rotation, factor);
    Some(rotation)
}
