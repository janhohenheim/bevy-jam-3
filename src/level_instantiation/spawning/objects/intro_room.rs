use crate::file_system_interaction::asset_loading::RoomAssets;
use crate::level_instantiation::spawning::GameObject;
use bevy::prelude::*;

pub(crate) fn spawn(In(transform): In<Transform>, mut commands: Commands, rooms: Res<RoomAssets>) {
    commands.spawn((
        SceneBundle {
            scene: rooms.intro.clone(),
            transform,
            ..default()
        },
        Name::new("Intro Room"),
        Imported,
        GameObject::IntroRoom,
    ));
}

#[derive(Component)]
pub(crate) struct Imported;
