use crate::combat::{AttackHitbox, Combatant, HitboxToParentLink, ParentToHitboxLink, Projectile};
use crate::level_instantiation::spawning::objects::GameCollisionGroup;
use crate::util::trait_extension::MeshExt;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;
use bevy_rapier3d::prelude::*;

#[sysfail(log(level = "error"))]
pub fn link_hitbox(
    mut commands: Commands,
    parents: Query<
        (Entity,),
        (
            Or<(With<Combatant>, With<Projectile>)>,
            Without<ParentToHitboxLink>,
        ),
    >,
    children: Query<&Children>,
    mesh_handles: Query<&Handle<Mesh>>,
    meshes: Res<Assets<Mesh>>,
    names: Query<&Name>,
) -> Result<()> {
    for (parent,) in parents.iter() {
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
                AttackHitbox::default(),
            ));
            commands.entity(parent).insert(ParentToHitboxLink(child));
            break;
        }
    }
    Ok(())
}

#[sysfail(log(level = "error"))]
pub fn sync_projectile_attack_hitbox(
    projectiles: Query<(&AttackHitbox, &ParentToHitboxLink)>,
    mut hitboxes: Query<(&mut AttackHitbox, &mut CollisionGroups), Without<ParentToHitboxLink>>,
) -> Result<()> {
    for (attack, link) in projectiles.iter() {
        let (mut hitbox, mut collision_groups) = hitboxes
            .get_mut(link.0)
            .context("ParentToHitboxLink of projectile holds invalid entity")?;
        *hitbox = *attack;
        if attack.active {
            collision_groups.filters |= GameCollisionGroup::PLAYER.into();
        } else {
            collision_groups.filters -= GameCollisionGroup::PLAYER.into();
        }
    }
    Ok(())
}
