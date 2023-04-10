use crate::combat::{Attack, Constitution, HitboxParentModel};
use crate::file_system_interaction::asset_loading::{FpsDummyAnimationAssets, SceneAssets};
use crate::level_instantiation::spawning::objects::GameCollisionGroup;
use crate::level_instantiation::spawning::GameObject;
use crate::movement::general_movement::{CharacterControllerBundle, ManualRotation, Model};
use crate::player_control::actions::{
    create_player_action_input_manager_bundle, create_ui_action_input_manager_bundle,
};
use crate::player_control::camera::IngameCamera;
use crate::player_control::player_embodiment::combat::{
    BlockHistory, CancellationTimes, PeriodicCancellationTimes, PlayerAttacks,
    PlayerCombatAnimation, PlayerCombatAnimations, PlayerCombatBundle,
};
use crate::player_control::player_embodiment::Player;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::TAU;

pub(crate) const HEIGHT: f32 = 0.4;
pub(crate) const RADIUS: f32 = 0.3;

pub(crate) fn spawn(
    In(transform): In<Transform>,
    mut commands: Commands,
    scene_handles: Res<SceneAssets>,
    animations: Res<FpsDummyAnimationAssets>,
    cameras: Query<Entity, With<IngameCamera>>,
) {
    let player_entity = commands
        .spawn((
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
            PlayerCombatBundle {
                player_combat: default(),
                player_combat_animations: PlayerCombatAnimations {
                    idle: PlayerCombatAnimation::always_cancellable(animations.idle.clone()),
                    attacks: [
                        PlayerCombatAnimation {
                            handle: animations.attack_one.clone(),
                            cancellation_times: CancellationTimes::Periodic(
                                PeriodicCancellationTimes {
                                    early_cancel_end: 0.15,
                                    late_cancel_start: 0.7,
                                    buffer_start: 0.5,
                                },
                            ),
                        },
                        PlayerCombatAnimation {
                            handle: animations.attack_two.clone(),
                            cancellation_times: CancellationTimes::Periodic(
                                PeriodicCancellationTimes {
                                    early_cancel_end: 0.1,
                                    late_cancel_start: 0.9,
                                    buffer_start: 0.5,
                                },
                            ),
                        },
                        PlayerCombatAnimation {
                            handle: animations.attack_three.clone(),
                            cancellation_times: CancellationTimes::Periodic(
                                PeriodicCancellationTimes {
                                    early_cancel_end: 0.07,
                                    late_cancel_start: 0.9,
                                    buffer_start: 0.7,
                                },
                            ),
                        },
                    ],
                    block: PlayerCombatAnimation::always_cancellable(animations.block.clone()),
                    hurt: PlayerCombatAnimation {
                        handle: animations.hurt.clone(),
                        cancellation_times: CancellationTimes::Periodic(
                            PeriodicCancellationTimes {
                                early_cancel_end: 0.0,
                                late_cancel_start: 0.3,
                                buffer_start: 0.2,
                            },
                        ),
                    },
                    deflected: PlayerCombatAnimation {
                        handle: animations.blocked.clone(),
                        cancellation_times: CancellationTimes::Periodic(
                            PeriodicCancellationTimes {
                                early_cancel_end: 0.0,
                                late_cancel_start: 0.9,
                                buffer_start: 0.7,
                            },
                        ),
                    },
                    posture_broken: PlayerCombatAnimation {
                        handle: animations.hurt.clone(),
                        cancellation_times: CancellationTimes::Periodic(
                            PeriodicCancellationTimes {
                                early_cancel_end: 0.0,
                                late_cancel_start: 0.7,
                                buffer_start: 0.5,
                            },
                        ),
                    },
                },
                player_attacks: PlayerAttacks {
                    attacks: [
                        Attack::new("Attack 1").with_health_damage_scaling_rest(10.0),
                        Attack::new("Attack 2").with_health_damage_scaling_rest(8.0),
                        Attack::new("Attack 3").with_health_damage_scaling_rest(15.0),
                    ],
                },
                constitution: Constitution::default()
                    .with_max_health(100.0)
                    .with_max_posture(50.0)
                    .with_base_posture_recovery(8.0),
                block_history: BlockHistory::default(),
            },
            GameObject::Player,
        ))
        .id();

    commands
        .spawn((
            HitboxParentModel,
            Model {
                follow_target: cameras.single(),
                animation_target: player_entity,
            },
            SpatialBundle::default(),
            Name::new("Player Model Parent"),
        ))
        .with_children(|parent| {
            parent.spawn((
                SceneBundle {
                    scene: scene_handles.fps_dummy.clone(),
                    transform: Transform {
                        translation: Vec3::new(-0.05, -0.15, -0.2),
                        rotation: Quat::from_rotation_y(TAU / 2.),
                        scale: Vec3::new(0.2, 0.2, 0.4),
                    },
                    ..default()
                },
                Name::new("Player Model"),
            ));
        });
}
