use crate::ai::generic::motion::asymptotic_rotation_to_horizontal;
use crate::combat::{MotionFn, MotionFnInput, MotionFnOutput};
use crate::util::trait_extension::Vec3Ext;
use bevy::prelude::*;
use bevy::utils::AHasher;
use bevy_rapier3d::prelude::*;
use std::hash::Hasher;

use noise::{core::perlin::perlin_1d, permutationtable::PermutationTable};

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
                  start_player_direction,
                  mass,
                  config,
                  dt,
                  global_time,
                  ..
              }: MotionFnInput| {
            let noise = generate_noise(start_player_direction, global_time);
            let force = (!player_direction.is_approx_zero())
                .then_some(
                    player_direction.normalize().cross(Vec3::Y) * acceleration * mass * noise,
                )
                .unwrap_or_default();
            let smoothness = config.characters.rotation_smoothing;
            let rotation =
                asymptotic_rotation_to_horizontal(transform, player_direction, smoothness, dt);
            MotionFnOutput {
                force: ExternalForce { force, ..default() },
                rotation,
                ..default()
            }
        },
    )
}

fn generate_noise(seed_source: Vec3, global_time: f32) -> f32 {
    let mut hasher = AHasher::default();
    hasher.write(bytemuck::bytes_of(&seed_source));
    let seed = hasher.finish();
    let permutation_table = PermutationTable::new(seed as u32);
    perlin_1d(global_time as f64 / 3., &permutation_table) as f32
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
