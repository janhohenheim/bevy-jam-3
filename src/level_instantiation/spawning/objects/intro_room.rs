use crate::file_system_interaction::asset_loading::RoomAssets;
use crate::level_instantiation::spawning::GameObject;
use crate::world_interaction::room::Room;
use bevy::prelude::*;

pub(crate) fn spawn_intro(
    In(transform): In<Transform>,
    mut commands: Commands,
    rooms: Res<RoomAssets>,
) {
    commands.spawn((
        SceneBundle {
            scene: rooms.intro.clone(),
            transform,
            ..default()
        },
        Name::new("Intro Room"),
        Imported,
        GameObject::IntroRoom,
        Room,
    ));
}

pub(crate) fn spawn_one(
    In(transform): In<Transform>,
    mut commands: Commands,
    rooms: Res<RoomAssets>,
) {
    commands.spawn((
        SceneBundle {
            scene: rooms.one.clone(),
            transform,
            ..default()
        },
        Name::new("Room One"),
        Imported,
        GameObject::RoomOne,
        Room,
    ));
}

pub(crate) fn spawn_two(
    In(transform): In<Transform>,
    mut commands: Commands,
    rooms: Res<RoomAssets>,
) {
    commands.spawn((
        SceneBundle {
            scene: rooms.two.clone(),
            transform,
            ..default()
        },
        Name::new("Room Two"),
        Imported,
        GameObject::RoomTwo,
        Room,
    ));
}

pub(crate) fn spawn_three(
    In(transform): In<Transform>,
    mut commands: Commands,
    rooms: Res<RoomAssets>,
) {
    commands.spawn((
        SceneBundle {
            scene: rooms.three.clone(),
            transform,
            ..default()
        },
        Name::new("Room Three"),
        Imported,
        GameObject::RoomThree,
        Room,
    ));
}

#[derive(Component)]
pub(crate) struct Imported;
