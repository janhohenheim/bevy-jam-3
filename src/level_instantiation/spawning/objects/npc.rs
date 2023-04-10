use crate::ai;
use crate::combat::components::*;
use crate::file_system_interaction::asset_loading::{DummyAnimationAssets, SceneAssets};
use crate::level_instantiation::spawning::objects::GameCollisionGroup;
use crate::level_instantiation::spawning::GameObject;
use crate::movement::general_movement::{CharacterControllerBundle, Model};
use crate::world_interaction::room::Room;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_rapier3d::prelude::*;
use std::f32::consts::TAU;

pub(crate) const HEIGHT: f32 = 0.4;
pub(crate) const RADIUS: f32 = 0.4;

pub(crate) fn spawn(
    In(transform): In<Transform>,
    mut commands: Commands,
    animations: Res<DummyAnimationAssets>,
    scene_handles: Res<SceneAssets>,
) {
    let entity = commands
        .spawn((
            PbrBundle {
                transform,
                ..default()
            },
        Room,
            Name::new("NPC"),
            CharacterControllerBundle::capsule(HEIGHT, RADIUS),
            CombatBundle {
                enemy: Enemy::new(vec![
                    Choreography {
                        name: "Walk toward Player".to_string(),
                        moves: vec![Move {
                            metadata: MoveMetadata {
                                duration: MoveDuration::While(
                                    CombatCondition::PlayerDistanceOver(2.0),
                                ),
                                animation: Some(animations.walk.clone()),
                                state: EnemyCombatState::OnGuard,
                            },
                            functions: MoveFunctions {
                                motion_fn: Some(
                                    ai::generic::motion::continuous::accelerate_towards_player(
                                        16.,
                                    ),
                                ),
                                ..default()
                            },
                            ..default()
                        }],
                    },
                    Choreography {
                        name: "Idle".to_string(),
                        moves: vec![Move {
                            metadata: MoveMetadata {
                                duration: MoveDuration::Fixed(2.0),
                                animation: Some(animations.idle.clone()),
                                state: EnemyCombatState::OnGuard,
                            },
                            functions: MoveFunctions {
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
                                metadata: MoveMetadata {
                                    duration: MoveDuration::Fixed(0.3),
                                    animation: Some(animations.attack.clone()),
                                    state: EnemyCombatState::OnGuard,
                                },
                                ..default()
                            },
                            Move {
                                name: Some("Dash".to_string()),
                                metadata: MoveMetadata {
                                    duration: MoveDuration::Instant,
                                    state: EnemyCombatState::Vulnerable,
                                    ..default()
                                },
                                functions: MoveFunctions {
                                    motion_fn: Some(
                                        ai::generic::motion::instant::step_toward_player(8.),
                                    ),
                                    ..default()
                                },
                            },
                            Move {
                                name: Some("Attack".to_string()),
                                metadata: MoveMetadata {
                                    duration: MoveDuration::Fixed(0.3),
                                    state: EnemyCombatState::Vulnerable,
                                    ..default()
                                },
                                functions: MoveFunctions {
                                    melee_attack_fn: Some(ai::generic::melee::whole_animation(
                                        Attack::new("Default NPC Attack").with_health_damage_scaling_rest(10.).with_posture_damage(6.),
                                    )),
                                    ..default()
                                },
                            },
                            Move {
                                name: Some("Attack finish".to_string()),
                                metadata: MoveMetadata {
                                    duration: MoveDuration::Animation,
                                    state: EnemyCombatState::Vulnerable,
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
                                metadata: MoveMetadata {
                                    duration: MoveDuration::Instant,
                                    state: EnemyCombatState::Vulnerable,
                                    ..default()
                                },
                                functions: MoveFunctions {
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
                                metadata: MoveMetadata {
                                    duration: MoveDuration::Fixed(0.25),
                                    state: EnemyCombatState::Vulnerable,
                                    animation: Some(animations.aerial.clone()),
                                },
                                ..default()
                            },
                            Move {
                                name: Some("Toss Kunai".to_string()),
                                metadata: MoveMetadata {
                                    duration: MoveDuration::Fixed(0.2),
                                    state: EnemyCombatState::Vulnerable,
                                    animation: Some(animations.aerial_toss.clone()),
                                },
                                functions: MoveFunctions {
                                    motion_fn: Some(ai::generic::motion::continuous::face_player()),
                                    ..default()
                                },
                            },
                            Move {
                                name: Some("Spawn Kunai".to_string()),
                                metadata: MoveMetadata {
                                    duration: MoveDuration::Instant,
                                    state: EnemyCombatState::Vulnerable,
                                    ..default()
                                },
                                functions: MoveFunctions {
                                    projectile_attack_fn: Some(
                                        ai::generic::projectile::spawn_simple_projectile(
                                            ProjectileSpawnInput {
                                                model: scene_handles.kunai.clone(),
                                                attack: AttackHitbox::from_attack(Attack::new("Kunai Throw").with_health_damage_scaling_rest(10.0)),
                                                speed: 10.0,
                                                tracking: 0.5,
                                                max_lifetime: 3.0,
                                            },
                                        ),
                                    ),
                                    ..default()
                                },
                            },
                            Move {
                                name: Some("Finish Toss Kunai".to_string()),
                                metadata: MoveMetadata {
                                    duration: MoveDuration::Animation,
                                    state: EnemyCombatState::Vulnerable,
                                    animation: Some(animations.aerial_toss.clone()),
                                },
                                functions: MoveFunctions {
                                    motion_fn: Some(ai::generic::motion::continuous::face_player()),
                                    ..default()
                                },
                            },
                            Move {
                                name: Some("Fall".to_string()),
                                metadata: MoveMetadata {
                                    duration: MoveDuration::Until(CombatCondition::Grounded),
                                    state: EnemyCombatState::Vulnerable,
                                    animation: Some(animations.aerial.clone()),
                                },
                                ..default()
                            },
                            Move {
                                name: Some("Land vulnerability".to_string()),
                                metadata: MoveMetadata {
                                    duration: MoveDuration::Fixed(0.5),
                                    state: EnemyCombatState::Vulnerable,
                                    animation: Some(animations.idle.clone()),
                                },
                                functions: MoveFunctions {
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
                    Choreography {
                        name: "Circle Around Player".to_string(),
                        moves: vec![
                            Move {
                                name: None,
                                metadata: MoveMetadata {
                                    duration: MoveDuration::Fixed(4.),
                                    animation: Some(animations.walk.clone()),
                                    state: EnemyCombatState::OnGuard
                                },
                                functions: MoveFunctions {
                                    motion_fn: Some(
                                        ai::generic::motion::continuous::accelerate_around_player(
                                            12.,
                                        ),
                                    ),
                                    ..default()
                                },
                            }
                        ]
                    },
                    Choreography {
                        name: "Block".to_string(),
                        moves: vec![
                            Move {
                                name: Some("Knockback".to_string()),
                                metadata: MoveMetadata {
                                    duration: MoveDuration::Instant,
                                    animation: Some(animations.block.clone()),
                                    state: EnemyCombatState::OnGuard
                                },
                                functions: MoveFunctions {
                                    motion_fn: Some(
                                        ai::generic::motion::instant::step_toward_player(
                                            -3.,
                                        ),
                                    ),
                                    ..default()
                                },
                            },
                            Move {
                                name: Some("Block".to_string()),
                                metadata: MoveMetadata {
                                    duration: MoveDuration::Fixed(0.5),
                                    animation: None,
                                    state: EnemyCombatState::OnGuard
                                },
                                functions: MoveFunctions {
                                    motion_fn: Some(
                                        ai::generic::motion::continuous::face_player_with_smoothness(0.2),
                                    ),
                                    ..default()
                                },
                            }
                        ]
                    },
                    Choreography {
                        name: "Hurt".to_string(),
                        moves: vec![
                            Move {
                                name: Some("Knockback".to_string()),
                                metadata: MoveMetadata {
                                    duration: MoveDuration::Instant,
                                    animation: Some(animations.hurt.clone()),
                                    state: EnemyCombatState::OnGuard
                                },
                                functions: MoveFunctions {
                                    motion_fn: Some(
                                        ai::generic::motion::instant::step_toward_player(
                                            -3.,
                                        ),
                                    ),
                                    ..default()
                                },
                            },
                            Move {
                                name: Some("Recover".to_string()),
                                metadata: MoveMetadata {
                                    duration: MoveDuration::Animation,
                                    animation: None,
                                    state: EnemyCombatState::OnGuard
                                },
                                functions: MoveFunctions {
                                    motion_fn: Some(
                                        ai::generic::motion::continuous::face_player_with_smoothness(0.3),
                                    ),
                                    ..default()
                                },
                            }
                        ]
                    },
                    Choreography {
                        name: "Posture broken".to_string(),
                        moves: vec![
                            Move {
                                metadata: MoveMetadata {
                                    duration: MoveDuration::Fixed(4.0),
                                    animation: Some(animations.hurt.clone()),
                                    state: EnemyCombatState::Deathblow
                                },
                                ..default()
                            }
                        ]
                    },
                    Choreography {
                        name: "Death".to_string(),
                        moves: vec![
                            Move {
                                metadata: MoveMetadata {
                                    duration: MoveDuration::While(CombatCondition::True),
                                    animation: Some(animations.hurt.clone()),
                                    state: EnemyCombatState::Dying
                                },
                                ..default()
                            }
                        ]
                    }
                ], vec![
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
                    Tendency {
                        // Circle around player
                        choreography: 4,
                        weight: 0.2,
                        condition: CombatCondition::True
                    }
                ], HashMap::new(), SpecialChoreographies {
                    block: 5,
                    hurt: 6,
                    posture_broken: 7,
                    death: 8,
                }),
                constitution: Constitution::default().with_max_health(100.0).with_max_posture(50.0).with_base_posture_recovery(10.0),
                ..default()
            },
            GameObject::Dummy,
            CollisionGroups::new(
                GameCollisionGroup::ENEMY.into(),
                (GameCollisionGroup::PLAYER | GameCollisionGroup::ATTACK).into(),
            ),
        )).id();
    commands
        .spawn((
            HitboxParentModel,
            Model::with_same_follow_and_animation_targets(entity),
            SpatialBundle::default(),
            Name::new("Player Model Parent"),
        ))
        .with_children(|parent| {
            parent.spawn((SceneBundle {
                scene: scene_handles.dummy.clone(),
                transform: Transform {
                    translation: Vec3::new(0., -HEIGHT / 2. - RADIUS, 0.),
                    scale: Vec3::splat(0.25),
                    rotation: Quat::from_rotation_y(TAU / 2.),
                },
                ..default()
            },));
        });
}
