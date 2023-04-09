pub(crate) mod continuous;
pub(crate) mod instant;
use crate::util::smoothness_to_lerp_factor;
use crate::util::trait_extension::Vec3Ext;
use bevy::prelude::*;

fn asymptotic_rotation_to_horizontal(
    transform: Transform,
    direction: Vec3,
    smoothness: f32,
    dt: f32,
) -> Option<Quat> {
    let target_rotation = rotation_to_horizontal(transform, direction)?;
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
