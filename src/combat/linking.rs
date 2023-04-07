use crate::combat::{AttackHitbox, Combatant, HitboxToParentLink, ParentToHitboxLink, Projectile};
use crate::level_instantiation::spawning::objects::GameCollisionGroup;
use crate::movement::general_movement::Model;
use crate::player_control::player_embodiment::PlayerModel;
use crate::util::trait_extension::MeshExt;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;
use bevy_rapier3d::prelude::*;

#[sysfail(log(level = "error"))]
pub fn link_hitbox(
    mut commands: Commands,
    parents: Query<
        (Entity, Option<&Model>),
        (
            Or<(With<Combatant>, With<Projectile>, With<PlayerModel>)>,
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
            .1;
        let collider = Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh)
            .context("Failed to create collider from mesh")?;
        commands.entity(bone_child).insert((
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
            HitboxToParentLink(parent),
            AttackHitbox::default(),
        ));
        commands
            .entity(parent)
            .insert(ParentToHitboxLink(bone_child));
        if let Some(model) = model {
            commands
                .entity(model.animation_target)
                .insert(ParentToHitboxLink(bone_child));
        }
    }
    Ok(())
}

#[sysfail(log(level = "error"))]
pub fn sync_projectile_attack_hitbox(
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
