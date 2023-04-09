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
            let smoothness = config.characters.rotation_smoothing;
            let rotation = asymptotic_rotation_to_horizontal(transform, velocity, smoothness, dt);
            MotionFnOutput {
                force: ExternalForce { force, ..default() },
                rotation,
                ..default()
            }
        },
    )
}

pub fn accelerate_around_player(acceleration: f32) -> Box<dyn MotionFn> {
    Box::new(
        move |MotionFnInput {
                  transform,
                  player_direction,
                  mass,
                  velocity,
                  config,
                  dt,
                  ..
              }: MotionFnInput| {
            let force = (!player_direction.is_approx_zero())
                .then_some(player_direction.cross(Vec3::Y).normalize() * acceleration * mass)
                .unwrap_or_default();
            let force = if rand::random() { -force } else { force };
            let smoothness = config.characters.rotation_smoothing;
            let rotation = asymptotic_rotation_to_horizontal(transform, velocity, smoothness, dt);
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
            let smoothness = config.characters.rotation_smoothing;
            let rotation =
                asymptotic_rotation_to_horizontal(transform, player_direction, smoothness, dt);
            MotionFnOutput {
                rotation,
                ..default()
            }
        },
    )
}

pub fn face_player_with_smoothness(smoothness: f32) -> Box<dyn MotionFn> {
    Box::new(
        move |MotionFnInput {
                  transform,
                  player_direction,
                  dt,
                  ..
              }: MotionFnInput| {
            let rotation =
                asymptotic_rotation_to_horizontal(transform, player_direction, smoothness, dt);
            MotionFnOutput {
                rotation,
                ..default()
            }
        },
    )
}
