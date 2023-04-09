use crate::combat::{
    AttackHitbox, HitboxParentModel, HitboxToParentLink, ParentToHitboxLink, Projectile,
};
use crate::level_instantiation::spawning::objects::GameCollisionGroup;
use crate::movement::general_movement::Model;
use crate::util::trait_extension::MeshExt;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;
use bevy_rapier3d::prelude::*;

#[sysfail(log(level = "error"))]
pub(crate) fn link_hitbox(
    mut commands: Commands,
    parents: Query<
        (Entity, Option<&Model>),
        (
            Or<(With<HitboxParentModel>, With<Projectile>)>,
            Without<ParentToHitboxLink>,
        ),
    >,
    children: Query<&Children>,
    mesh_handles: Query<&Handle<Mesh>>,
    meshes: Res<Assets<Mesh>>,
    names: Query<&Name>,
) -> Result<()> {
    for (parent, model) in parents.iter() {
        let mut mesh_child = None;
        let mut bone_child = None;
        for child in children.iter_descendants(parent) {
            let Ok(name) = names.get(child) else {
                continue;
            };
            if name.contains("[hitbox]") {
                mesh_child = Some(child);
            }
            if name.contains("[hitbox-bone]") {
                bone_child = Some(child);
            }
            if mesh_child.is_some() && bone_child.is_some() {
                break;
            }
        }
        let Some(mesh_child) = mesh_child else {
            continue;
        };
        let bone_child = bone_child.unwrap_or(mesh_child);
        let mesh = Mesh::search_in_children(mesh_child, &children, &meshes, &mesh_handles)
            .first()
            .context("Hitbox entity has no mesh")?
            .1
            .clone();
        let aabb = mesh
            .compute_aabb()
            .context("Failed to compute AABB of mesh")?;
        let collider = Collider::cuboid(
            aabb.half_extents.x,
            aabb.half_extents.y,
            aabb.half_extents.z,
        );

        let true_parent = if let Some(model) = model {
            model.animation_target
        } else {
            parent
        };
        let collider_entity = commands
            .spawn((
                Name::new("Hitbox collider"),
                collider,
                CollisionGroups::new(
                    GameCollisionGroup::ATTACK.into(),
                    GameCollisionGroup::NONE.into(),
                ),
                SolverGroups {
                    memberships: GameCollisionGroup::ATTACK.into(),
                    filters: GameCollisionGroup::NONE.into(),
                },
                ActiveEvents::COLLISION_EVENTS,
                ActiveCollisionTypes::all(),
                HitboxToParentLink(true_parent),
                AttackHitbox::default(),
                TransformBundle::from_transform(Transform::from_xyz(0., aabb.half_extents.y, 0.0)),
            ))
            .id();
        commands.entity(bone_child).add_child(collider_entity);
        commands
            .entity(parent) // only done to stop query from spinning
            .insert(ParentToHitboxLink(collider_entity));
        commands
            .entity(true_parent)
            .insert(ParentToHitboxLink(collider_entity));
    }
    Ok(())
}

#[sysfail(log(level = "error"))]
pub(crate) fn sync_projectile_attack_hitbox(
    projectiles: Query<(&AttackHitbox, &ParentToHitboxLink), With<Projectile>>,
    mut hitboxes: Query<(&mut AttackHitbox, &mut CollisionGroups), Without<ParentToHitboxLink>>,
) -> Result<()> {
    for (attack, link) in projectiles.iter() {
        let (mut hitbox, mut collision_groups) = hitboxes
            .get_mut(link.0)
            .context("ParentToHitboxLink of projectile holds invalid entity")?;
        *hitbox = attack.clone();
        if attack.active {
            collision_groups.filters |= GameCollisionGroup::PLAYER.into();
        } else {
            collision_groups.filters -= GameCollisionGroup::PLAYER.into();
        }
    }
    Ok(())
}
