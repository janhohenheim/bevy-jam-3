use crate::file_system_interaction::asset_loading::SceneAssets;
use crate::level_instantiation::spawning::objects::GameCollisionGroup;
use crate::level_instantiation::spawning::GameObject;
use crate::movement::general_movement::{CharacterControllerBundle, ManualRotation, Model};
use crate::player_control::actions::{
    create_player_action_input_manager_bundle, create_ui_action_input_manager_bundle,
};
use crate::player_control::camera::IngameCamera;
use crate::player_control::player_embodiment::combat::PlayerCombatState;
use crate::player_control::player_embodiment::Player;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::TAU;

pub const HEIGHT: f32 = 0.4;
pub const RADIUS: f32 = 0.3;

pub(crate) fn spawn(
    In(transform): In<Transform>,
    mut commands: Commands,
    scene_handles: Res<SceneAssets>,
    cameras: Query<Entity, With<IngameCamera>>,
) {
    commands.spawn((
        PbrBundle {
            transform,
            ..default()
        },
        Player,
        Name::new("Player"),
        Ccd::enabled(),
        ManualRotation,
        CharacterControllerBundle::capsule(HEIGHT, RADIUS),
        CollisionGroups::new(
            GameCollisionGroup::PLAYER.into(),
            GameCollisionGroup::ALL.into(),
        ),
        create_player_action_input_manager_bundle(),
        create_ui_action_input_manager_bundle(),
        PlayerCombatState::default(),
        GameObject::Player,
    ));

    commands
        .spawn((
            Model {
                target: cameras.single(),
            },
            SpatialBundle::default(),
            Name::new("Player Model Parent"),
        ))
        .with_children(|parent| {
            parent.spawn((
                SceneBundle {
                    scene: scene_handles.fps_dummy.clone(),
                    transform: Transform {
                        translation: Vec3::new(0., -0.1, -0.2),
                        rotation: Quat::from_rotation_y(TAU / 2.),
                        scale: Vec3::splat(0.2),
                    },
                    ..default()
                },
                Name::new("Player Model"),
            ));
        });
}
