use crate::level_instantiation::spawning::GameObject;

use bevy::prelude::*;

pub(crate) fn spawn(In(transform): In<Transform>, mut commands: Commands) {
    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                color: Color::hex("FD5D2C").unwrap(),
                range: 40.0,
                radius: 0.1,
                intensity: 100.0,
                shadows_enabled: true,
                ..default()
            },
            transform,
            ..default()
        },
        Name::new("Light"),
        GameObject::PointLight,
    ));
}
