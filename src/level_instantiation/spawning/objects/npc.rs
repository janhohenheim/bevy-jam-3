use crate::ai;
use crate::combat::components::*;
use crate::file_system_interaction::asset_loading::{DummyAnimationAssets, SceneAssets};
use crate::level_instantiation::spawning::objects::GameCollisionGroup;
use crate::level_instantiation::spawning::GameObject;
use crate::movement::general_movement::CharacterControllerBundle;
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
    animations: Res<DummyAnimationAssets>,
    scene_handles: Res<SceneAssets>,
) {
    commands
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
                                duration: MoveDuration::While(CombatCondition::PlayerDistanceOver(
                                    2.0,
                                )),
                                animation: Some(animations.walk.clone()),
                                state: CombatantState::OnGuard,
                            },
                            execute: ExecuteMove {
                                motion_fn: Some(
                                    ai::generic::motion::continuous::accelerate_towards_player(16.),
                                ),
                                ..default()
                            },
                            ..default()
                        }],
                    },
                    Choreography {
                        name: "Idle".to_string(),
                        moves: vec![Move {
                            init: InitMove {
                                duration: MoveDuration::Fixed(2.0),
                                animation: Some(animations.idle.clone()),
                                state: CombatantState::OnGuard,
                            },
                            execute: ExecuteMove {
                                motion_fn: Some(ai::generic::motion::continuous::face_player()),
                                ..default()
                            },
                            ..default()
                        }],
                    },
                    Choreography {
                        name: "Ground Attack".to_string(),
                        moves: vec![
                            Move {
                                name: Some("Hold up weapon".to_string()),
                                init: InitMove {
                                    duration: MoveDuration::Fixed(0.3),
                                    animation: Some(animations.attack.clone()),
                                    state: CombatantState::OnGuard,
                                },
                                ..default()
                            },
                            Move {
                                name: Some("Dash".to_string()),
                                init: InitMove {
                                    duration: MoveDuration::Instant,
                                    state: CombatantState::Vulnerable,
                                    ..default()
                                },
                                execute: ExecuteMove {
                                    motion_fn: Some(
                                        ai::generic::motion::instant::step_toward_player(8.),
                                    ),
                                    ..default()
                                },
                            },
                            Move {
                                name: Some("Attack".to_string()),
                                init: InitMove {
                                    duration: MoveDuration::Fixed(0.3),
                                    state: CombatantState::Vulnerable,
                                    ..default()
                                },
                                execute: ExecuteMove {
                                    melee_attack_fn: Some(ai::generic::melee::whole_animation(
                                        Attack {
                                            damage: 10.0,
                                            knockback: 5.0,
                                        },
                                    )),
                                    ..default()
                                },
                            },
                            Move {
                                name: Some("Attack finish".to_string()),
                                init: InitMove {
                                    duration: MoveDuration::Animation,
                                    state: CombatantState::Vulnerable,
                                    ..default()
                                },
                                ..default()
                            },
                        ],
                    },
                    Choreography {
                        name: "Air Attack".to_string(),
                        moves: vec![
                            Move {
                                name: Some("Jump impulse".to_string()),
                                init: InitMove {
                                    duration: MoveDuration::Instant,
                                    state: CombatantState::Vulnerable,
                                    ..default()
                                },
                                execute: ExecuteMove {
                                    motion_fn: Some(
                                        ai::generic::motion::instant::jump_relative_to_player(
                                            10., 45.,
                                        ),
                                    ),
                                    ..default()
                                },
                            },
                            Move {
                                name: Some("Jump".to_string()),
                                init: InitMove {
                                    duration: MoveDuration::Fixed(0.25),
                                    state: CombatantState::Vulnerable,
                                    animation: Some(animations.aerial.clone()),
                                },
                                ..default()
                            },
                            Move {
                                name: Some("Toss Kunai".to_string()),
                                init: InitMove {
                                    duration: MoveDuration::Fixed(0.2),
                                    state: CombatantState::Vulnerable,
                                    animation: Some(animations.aerial_toss.clone()),
                                },
                                execute: ExecuteMove {
                                    motion_fn: Some(ai::generic::motion::continuous::face_player()),
                                    ..default()
                                },
                            },
                            Move {
                                name: Some("Spawn Kunai".to_string()),
                                init: InitMove {
                                    duration: MoveDuration::Instant,
                                    state: CombatantState::Vulnerable,
                                    ..default()
                                },
                                execute: ExecuteMove {
                                    projectile_attack_fn: Some(
                                        ai::generic::projectile::spawn_simple_projectile(
                                            ProjectileSpawnInput {
                                                model: scene_handles.kunai.clone(),
                                                attack: AttackHitbox::from_attack(Attack {
                                                    damage: 5.0,
                                                    knockback: 10.0,
                                                }),
                                                speed: 10.0,
                                                tracking: 0.1,
                                                max_lifetime: 3.0,
                                            },
                                        ),
                                    ),
                                    ..default()
                                },
                            },
                            Move {
                                name: Some("Finish Toss Kunai".to_string()),
                                init: InitMove {
                                    duration: MoveDuration::Animation,
                                    state: CombatantState::Vulnerable,
                                    animation: Some(animations.aerial_toss.clone()),
                                },
                                execute: ExecuteMove {
                                    motion_fn: Some(ai::generic::motion::continuous::face_player()),
                                    ..default()
                                },
                            },
                            Move {
                                name: Some("Fall".to_string()),
                                init: InitMove {
                                    duration: MoveDuration::Until(CombatCondition::Grounded),
                                    state: CombatantState::Vulnerable,
                                    animation: Some(animations.aerial.clone()),
                                },
                                ..default()
                            },
                            Move {
                                name: Some("Land vulnerability".to_string()),
                                init: InitMove {
                                    duration: MoveDuration::Fixed(0.5),
                                    state: CombatantState::Vulnerable,
                                    animation: Some(animations.idle.clone()),
                                },
                                execute: ExecuteMove {
                                    motion_fn: Some(
                                        ai::generic::motion::continuous::accelerate_towards_player(
                                            14.,
                                        ),
                                    ),
                                    ..default()
                                },
                            },
                        ],
                    },
                ],
                vec![
                    Tendency {
                        // Walk toward player
                        choreography: 0,
                        weight: 2.0,
                        condition: CombatCondition::PlayerDistanceOver(2.0),
                    },
                    Tendency {
                        // Idle
                        choreography: 1,
                        weight: 1.0,
                        condition: CombatCondition::PlayerDistanceOver(3.0),
                    },
                    Tendency {
                        // Ground attack
                        choreography: 2,
                        weight: 2.0,
                        condition: CombatCondition::PlayerDistanceUnder(2.0),
                    },
                    Tendency {
                        // Air attack
                        choreography: 3,
                        weight: 0.5,
                        condition: CombatCondition::And(vec![
                            CombatCondition::PlayerDistanceUnder(1.5),
                            CombatCondition::Grounded,
                        ]),
                    },
                ],
                HashMap::new(),
            )),
            DialogTarget {
                dialog_id: DialogId::new("follower"),
            },
            GameObject::Npc,
            CollisionGroups::new(
                GameCollisionGroup::ENEMY.into(),
                GameCollisionGroup::PLAYER.into(),
            ),
        ))
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
        .with_children(|parent| {
            parent.spawn((
                Name::new("NPC Model"),
                SceneBundle {
                    scene: scene_handles.dummy.clone(),
                    transform: Transform {
                        translation: Vec3::new(0., -HEIGHT / 2. - RADIUS, 0.),
                        scale: Vec3::splat(0.25),
                        rotation: Quat::from_rotation_y(TAU / 2.),
                    },
                    ..default()
                },
            ));
        });
}
