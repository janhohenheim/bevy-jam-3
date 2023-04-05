use crate::combat::{MotionFn, MotionFnInput, MotionFnOutput};
use crate::file_system_interaction::config::GameConfig;
use crate::util::smoothness_to_lerp_factor;
use crate::util::trait_extension::Vec3Ext;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn accelerate_towards_player(acceleration: f32) -> Box<dyn MotionFn> {
    Box::new(
        move |MotionFnInput {
                  transform,
                  line_of_sight_direction,
                  mass,
                  velocity,
                  config,
                  dt,
                  ..
              }: MotionFnInput| {
            let force = (!line_of_sight_direction.is_approx_zero())
                .then_some(line_of_sight_direction.normalize() * acceleration * mass)
                .unwrap_or_default();
            let rotation = asymptotic_rotation_to_horizontal(transform, velocity, config, dt);
            MotionFnOutput {
                force: ExternalForce { force, ..default() },
                rotation,
                ..default()
            }
        },
    )
}

pub fn face_player() -> Box<dyn MotionFn> {
    Box::new(
        move |MotionFnInput {
                  transform,
                  player_direction,
                  config,
                  dt,
                  ..
              }: MotionFnInput| {
            let rotation =
                asymptotic_rotation_to_horizontal(transform, player_direction, config, dt);
            MotionFnOutput {
                rotation,
                ..default()
            }
        },
    )
}

pub fn step_toward_player(speed: f32) -> Box<dyn MotionFn> {
    Box::new(
        move |MotionFnInput {
                  transform,
                  start_player_direction,
                  mass,
                  ..
              }: MotionFnInput| {
            let impulse = (!start_player_direction.is_approx_zero())
                .then_some(start_player_direction.normalize() * speed * mass)
                .unwrap_or_default();
            let rotation = rotation_to_horizontal(transform, start_player_direction);
            MotionFnOutput {
                impulse: ExternalImpulse {
                    impulse,
                    ..default()
                },
                rotation,
                ..default()
            }
        },
    )
}

fn asymptotic_rotation_to_horizontal(
    transform: Transform,
    direction: Vec3,
    config: GameConfig,
    dt: f32,
) -> Option<Quat> {
    let target_rotation = rotation_to_horizontal(transform, direction)?;
    let smoothness = config.characters.rotation_smoothing;
    let factor = smoothness_to_lerp_factor(smoothness, dt);
    let rotation = transform.rotation.slerp(target_rotation, factor);
    Some(rotation)
}

fn rotation_to_horizontal(transform: Transform, direction: Vec3) -> Option<Quat> {
    let up = transform.up();
    let horizontal_direction = direction.split(up).horizontal;
    if horizontal_direction.is_approx_zero() {
        return None;
    }

    let target_rotation = transform.looking_to(horizontal_direction, up).rotation;
    Some(target_rotation)
}
