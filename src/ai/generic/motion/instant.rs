use crate::ai::generic::motion::rotation_to_horizontal;
use crate::combat::{MotionFn, MotionFnInput, MotionFnOutput};
use crate::util::trait_extension::Vec3Ext;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub(crate) fn step_toward_player(speed: f32) -> Box<dyn MotionFn> {
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

/// `angle_degrees`: 0 = upward, positive = away from player, negative = toward player
pub(crate) fn jump_relative_to_player(speed: f32, angle_degrees: f32) -> Box<dyn MotionFn> {
    Box::new(
        move |MotionFnInput {
                  transform,
                  start_player_direction,
                  mass,
                  ..
              }: MotionFnInput| {
            let up = transform.up();
            let impulse = (!start_player_direction.is_approx_zero())
                .then(|| {
                    let axis = start_player_direction.normalize().cross(up);
                    let jump_rotation = Quat::from_axis_angle(axis, angle_degrees.to_radians());
                    let jump_direction = jump_rotation * up;
                    jump_direction * speed * mass
                })
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
