use crate::combat::{Combatant, HitboxToParentLink, MeleeAttack, ParentToHitboxLink};
use crate::level_instantiation::spawning::objects::GameCollisionGroup;
use crate::util::trait_extension::MeshExt;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;
use bevy_rapier3d::prelude::*;

#[sysfail(log(level = "error"))]
pub fn link_hitbox(
    mut commands: Commands,
    hitboxes: Query<(Entity,), (With<Combatant>, Without<ParentToHitboxLink>)>,
    children: Query<&Children>,
    mesh_handles: Query<&Handle<Mesh>>,
    meshes: Res<Assets<Mesh>>,
    names: Query<&Name>,
) -> Result<()> {
    for (parent,) in hitboxes.iter() {
        for child in children.iter_descendants(parent) {
            let Ok(name) = names.get(child) else {
                continue;
            };
            if !name.to_lowercase().contains("[hitbox]") {
                continue;
            }
            let mesh = Mesh::search_in_children(child, &children, &meshes, &mesh_handles)
                .first()
                .context("Hitbox entity has no mesh")?
                .1;
            let collider = Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh)
                .context("Failed to create collider from mesh")?;
            commands.entity(child).insert((
                collider,
                CollisionGroups::new(
                    GameCollisionGroup::ATTACK.into(),
                    GameCollisionGroup::NONE.into(),
                ),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                ActiveCollisionTypes::DYNAMIC_DYNAMIC,
                HitboxToParentLink(parent),
                MeleeAttack::default(),
            ));
            commands.entity(parent).insert(ParentToHitboxLink(child));
            break;
        }
    }
    Ok(())
}
