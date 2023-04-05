use crate::ai::generic::projectile::components::SimpleProjectile;
use crate::player_control::player_embodiment::Player;
use crate::util::smoothness_to_lerp_factor;
use bevy::prelude::*;

pub fn fly_toward_player(
    time: Res<Time>,
    mut projectiles: Query<(&mut Transform, &SimpleProjectile)>,
    players: Query<(&Transform,), (With<Player>, Without<SimpleProjectile>)>,
) {
    let dt = time.delta_seconds();
    for (mut transform, projectile) in projectiles.iter_mut() {
        for (player_transform,) in players.iter() {
            let current_direction = transform.forward();
            let tracking_direction =
                (player_transform.translation - transform.translation).normalize();
            let speed = projectile.speed;
            let smoothing = (1.0 - projectile.tracking) * 100.0;
            let factor = smoothness_to_lerp_factor(smoothing, dt);
            let direction = current_direction.lerp(tracking_direction, factor);
            transform.translation += direction * speed * dt;
            let up = transform.up();
            transform.look_to(direction, up);
        }
    }
}

pub fn handle_projectile_lifetimes(
    time: Res<Time>,
    mut commands: Commands,
    mut projectiles: Query<(Entity, &mut SimpleProjectile)>,
) {
    for (entity, mut projectile) in projectiles.iter_mut() {
        projectile.current_lifetime += time.delta_seconds();
        if projectile.current_lifetime > projectile.max_lifetime {
            commands.entity(entity).despawn_recursive();
        }
    }
}
