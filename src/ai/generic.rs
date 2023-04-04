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
                  line_of_sight_direction,
                  mass,
                  velocity,
                  config,
                  dt,
                  ..
              }: ForceFnInput| {
            let force = (!line_of_sight_direction.is_approx_zero())
                .then_some(line_of_sight_direction.normalize() * acceleration * mass)
                .unwrap_or_default();
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

pub fn step_toward_player(peak_acceleration: f32) -> Box<dyn ForceFn> {
    Box::new(
        move |ForceFnInput {
                  transform,
                  time,
                  duration,
                  start_player_direction,
                  mass,
                  dt,
                  config,
                  ..
              }: ForceFnInput| {
            let time_fraction = time / duration.unwrap();
            let peak_fraction = 0.5;
            let acceleration =
                parabola_through_origin(time_fraction, Vec2::new(peak_fraction, peak_acceleration));
            let force = (!start_player_direction.is_approx_zero())
                .then_some(start_player_direction.normalize() * acceleration * mass)
                .unwrap_or_default();
            let rotation = rotation_to_horizontal(transform, start_player_direction, config, dt);
            ForceFnOutput {
                force: ExternalForce { force, ..default() },
                rotation,
                ..default()
            }
        },
    )
}

fn parabola_through_origin(x: f32, vertex: Vec2) -> f32 {
    unimplemented!()
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
