use crate::level_instantiation::spawning::GameObject;

use crate::level_instantiation::spawning::objects::GameCollisionGroup;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Debug, Clone, Component)]
pub(crate) struct Exit;

pub(crate) fn spawn(In(transform): In<Transform>, mut commands: Commands) {
    commands
        .spawn((
            Name::new("Exit"),
            Exit,
            SpatialBundle::from_transform(transform),
            Collider::cylinder(1., 2.),
            RigidBody::Fixed,
            GameObject::Exit,
            ActiveEvents::COLLISION_EVENTS,
            ActiveCollisionTypes::all(),
            CollisionGroups::new(
                GameCollisionGroup::OTHER.into(),
                GameCollisionGroup::PLAYER.into(),
            ),
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Exit Collider"),
                Collider::cylinder(1., 2.),
                Sensor,
                RigidBody::Fixed,
                ActiveEvents::COLLISION_EVENTS,
                ActiveCollisionTypes::all(),
                CollisionGroups::new(
                    GameCollisionGroup::OTHER.into(),
                    GameCollisionGroup::PLAYER.into(),
                ),
            ));
        });
}
