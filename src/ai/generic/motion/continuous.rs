use crate::ai::generic::motion::asymptotic_rotation_to_horizontal;
use crate::combat::{MotionFn, MotionFnInput, MotionFnOutput};
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
