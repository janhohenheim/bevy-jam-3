use crate::ai::generic::projectile::components::SimpleProjectile;
use crate::combat::{
    ProjectileAttackFn, ProjectileAttackFnInput, ProjectileAttackFnOutput, ProjectileKind,
    ProjectileSpawnInput,
};
use crate::player_control::player_embodiment::Player;
use bevy::prelude::*;
use spew::prelude::SpawnEvent;

pub mod behavior;
pub mod components;

pub fn spawn_simple_projectile(input: ProjectileSpawnInput) -> Box<dyn ProjectileAttackFn> {
    Box::new(
        move |ProjectileAttackFnInput { spawner, .. }: ProjectileAttackFnInput| {
            let object = ProjectileKind::Simple;
            let input = (spawner, input.clone());
            let event = SpawnEvent::with_data(object, input);

            ProjectileAttackFnOutput {
                spawn_events: vec![event],
            }
        },
    )
}

pub fn spawn_actual_simple_projectile(
    In((
        spawner,
        ProjectileSpawnInput {
            speed,
            model,
            tracking,
            max_lifetime,
        },
    )): In<(Entity, ProjectileSpawnInput)>,
    mut commands: Commands,
    spawners: Query<(&Transform,), (Without<Player>,)>,
    players: Query<(&Transform,), (With<Player>, Without<SimpleProjectile>)>,
) {
    let (transform,) = spawners.get(spawner).unwrap();
    for (player_transform,) in players.iter() {
        let transform = transform
            .looking_at(player_transform.translation, transform.up())
            .with_scale(Vec3::splat(0.1));
        commands.spawn((
            Name::new("Projectile"),
            SimpleProjectile {
                speed,
                tracking,
                current_lifetime: 0.0,
                max_lifetime,
            },
            SceneBundle {
                scene: model.clone(),
                transform,
                ..Default::default()
            },
        ));
    }
}
