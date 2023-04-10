use crate::level_instantiation::spawning::GameObject;
use crate::world_interaction::room::{Exit, Room};
use bevy::prelude::*;

pub(crate) fn spawn(In(transform): In<Transform>, mut commands: Commands) {
    commands.spawn((
        Name::new("Exit"),
        Exit,
        SpatialBundle::from_transform(transform),
        GameObject::Exit,
        Room,
    ));
}
