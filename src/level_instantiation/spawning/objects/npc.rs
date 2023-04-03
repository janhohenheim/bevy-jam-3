use crate::ai;
use crate::combat::components::*;
use crate::file_system_interaction::asset_loading::{AnimationAssets, SceneAssets};
use crate::level_instantiation::spawning::objects::GameCollisionGroup;
use crate::level_instantiation::spawning::GameObject;
use crate::movement::general_movement::{CharacterAnimations, CharacterControllerBundle, Model};
use crate::world_interaction::dialog::{DialogId, DialogTarget};
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_rapier3d::prelude::*;
use std::f32::consts::TAU;

pub const HEIGHT: f32 = 0.4;
pub const RADIUS: f32 = 0.4;

pub(crate) fn spawn(
    In(transform): In<Transform>,
    mut commands: Commands,
    animations: Res<AnimationAssets>,
    scene_handles: Res<SceneAssets>,
) {
    let entity = commands
        .spawn((
            PbrBundle {
                transform,
                ..default()
            },
            Name::new("NPC"),
            CharacterControllerBundle::capsule(HEIGHT, RADIUS),
            CombatBundle::new(Combatant::new(
                vec![
                    Choreography {
                        name: "Walk toward Player".to_string(),
                        moves: vec![Move {
                            init: InitMove {
                                duration: MoveDuration::While(
                                    CombatCondition::PlayerDistanceSquaredOver(2.),
                                ),
                                animation: Some(animations.character_walking.clone()),
                                state: CombatantState::OnGuard,
                            },
                            execute: ExecuteMove {
                                force_fn: Some(ai::generic::accelerate_towards_player(14.)),
                                melee_attack_fn: None,
                            },
                        }],
                    },
                    Choreography {
                        name: "Idle".to_string(),
                        moves: vec![Move {
                            init: InitMove {
                                duration: MoveDuration::While(
                                    CombatCondition::PlayerDistanceSquaredUnder(2.),
                                ),
                                animation: Some(animations.character_idle.clone()),
                                state: CombatantState::OnGuard,
                            },
                            execute: ExecuteMove {
                                force_fn: None,
                                melee_attack_fn: None,
                            },
                        }],
                    },
                ],
                vec![
                    Tendency {
                        choreography: 0,
                        weight: 1.0,
                        condition: CombatCondition::PlayerDistanceSquaredOver(2.),
                    },
                    Tendency {
                        choreography: 1,
                        weight: 1.0,
                        condition: CombatCondition::PlayerDistanceSquaredUnder(2.),
                    },
                ],
                HashMap::new(),
            )),
            CharacterAnimations {
                idle: animations.character_idle.clone(),
                walk: animations.character_walking.clone(),
                aerial: animations.character_running.clone(),
            },
            DialogTarget {
                dialog_id: DialogId::new("follower"),
            },
            GameObject::Npc,
        ))
        .with_children(|parent| {
            parent.spawn((Name::new("NPC Melee Attack"), MeleeAttackBundle::default()));
        })
        .with_children(|parent| {
            parent.spawn((
                Name::new("NPC Dialog Collider"),
                Collider::cylinder(HEIGHT / 2., RADIUS * 5.),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                ActiveCollisionTypes::DYNAMIC_DYNAMIC,
                CollisionGroups::new(
                    GameCollisionGroup::OTHER.into(),
                    GameCollisionGroup::PLAYER.into(),
                ),
            ));
        })
        .id();

    commands
        .spawn((
            Model { target: entity },
            SpatialBundle::default(),
            Name::new("NPC Model Parent"),
        ))
        .with_children(|parent| {
            parent.spawn((
                SceneBundle {
                    scene: scene_handles.character.clone(),
                    transform: Transform {
                        translation: Vec3::new(0., -HEIGHT / 2. - RADIUS, 0.),
                        scale: Vec3::splat(0.012),
                        rotation: Quat::from_rotation_y(TAU / 2.),
                    },
                    ..default()
                },
                Name::new("NPC Model"),
            ));
        });
}
