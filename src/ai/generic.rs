use crate::combat::{ForceFn, ForceFnInput, ForceFnOutput};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn accelerate_towards_player(ground_acceleration: f32) -> Box<dyn ForceFn> {
    Box::new(
        move |ForceFnInput {
                  transform,
                  line_of_sight_path,
                  mass,
                  ..
              }: ForceFnInput| {
            let direction = (transform.translation - line_of_sight_path[0]).normalize();
            let force = direction * ground_acceleration * mass;
            ForceFnOutput {
                force: ExternalForce { force, ..default() },
                ..default()
            }
        },
    )
}
