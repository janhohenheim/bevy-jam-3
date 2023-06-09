use crate::combat::components::*;
use crate::file_system_interaction::config::GameConfig;
use crate::level_instantiation::spawning::objects::GameCollisionGroup;
use crate::level_instantiation::spawning::AnimationEntityLink;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;
use bevy_rapier3d::prelude::*;
use spew::prelude::SpawnEvent;
use std::time::Duration;

#[sysfail(log(level = "error"))]
pub(crate) fn read_move_metadata(
    mut move_events: EventReader<ReadMoveMetadataEvent>,
    mut animation_player: Query<&mut AnimationPlayer>,
    mut enemies: Query<(
        Option<&AnimationEntityLink>,
        &mut Enemy,
        &mut EnemyCombatState,
        &mut CurrentMoveMetadata,
        &ConditionTracker,
        &Transform,
    )>,
    animations: Res<Assets<AnimationClip>>,
) -> Result<()> {
    for event in move_events.iter() {
        let move_ = &event.move_;
        let (
            animation_entity_link,
            mut combatant,
            mut combatant_state,
            mut move_metadata,
            condition_tracker,
            transform,
        ) = enemies
            .get_mut(event.source)
            .context("Enemies did not have all required components during move metadata read")?;

        if let Some(animation_entity_link) = animation_entity_link {
            let mut animation_player = animation_player
                .get_mut(**animation_entity_link)
                .context("animation_entity_link held entity without animation player")?;

            if let Some(animation) = &move_.animation {
                combatant.time_since_last_animation = 0.0;
                animation_player
                    .play_with_transition(animation.clone(), Duration::from_secs_f32(0.2));
                animation_player.resume();
                if move_.duration != MoveDuration::Animation {
                    animation_player.repeat();
                }
            }
        }

        let animation_duration = move_
            .animation
            .as_ref()
            .and_then(|animation| animations.get(animation))
            .map(|animation| animation.duration())
            .or(move_metadata.animation_duration);

        *combatant_state = move_.state;
        *move_metadata = CurrentMoveMetadata {
            start_transform: *transform,
            start_player_direction: condition_tracker.player_direction,
            animation_duration,
        };
    }
    Ok(())
}

#[sysfail(log(level = "error"))]
pub(crate) fn execute_move_functions(
    time: Res<Time>,
    mut enemies: Query<(
        &Enemy,
        &ConditionTracker,
        &mut Transform,
        &ReadMassProperties,
        &mut ExternalForce,
        &mut ExternalImpulse,
        &CurrentMoveMetadata,
        &Velocity,
        &ParentToHitboxLink,
    )>,
    mut melee_attacks: Query<(&mut AttackHitbox, &mut CollisionGroups)>,
    mut move_events: EventReader<ExecuteMoveFunctionsEvent>,
    mut spawn_event_writer: EventWriter<SpawnEvent<ProjectileKind, (Entity, ProjectileSpawnInput)>>,
    game_config: Res<GameConfig>,
) -> Result<()> {
    for event in move_events.iter() {
        let entity = event.source;
        let (
            combatant,
            condition_tracker,
            mut transform,
            mass,
            mut force,
            mut impulse,
            move_metadata,
            velocity,
            hitbox_link,
        ) = enemies.get_mut(entity)?;
        if let Some(motion_fn) = &event.move_.motion_fn {
            let duration = match event.duration {
                MoveDuration::Animation => move_metadata.animation_duration,
                MoveDuration::Fixed(duration) => Some(duration),
                _ => None,
            };
            let input = MotionFnInput {
                _time_in_move: combatant.time_since_last_move,
                global_time: time.elapsed_seconds_wrapped(),
                _duration: duration,
                transform: *transform,
                _start_transform: move_metadata.start_transform,
                player_direction: condition_tracker.player_direction,
                start_player_direction: move_metadata.start_player_direction,
                _has_line_of_sight: condition_tracker.has_line_of_sight,
                line_of_sight_direction: condition_tracker.line_of_sight_direction,
                mass: mass.0.mass,
                velocity: velocity.linvel,
                config: game_config.clone(),
                dt: time.delta_seconds(),
            };
            let MotionFnOutput {
                force: output_force,
                impulse: output_impulse,
                rotation,
            } = motion_fn.call(input);
            *force += output_force;
            *impulse += output_impulse;
            if let Some(rotation) = rotation {
                transform.rotation = rotation;
            }
        }

        let (mut melee_attack, mut hitbox_collision_groups) =
            melee_attacks.get_mut(hitbox_link.0)?;
        if let Some(melee_attack_fn) = &event.move_.melee_attack_fn {
            let input = MeleeAttackFnInput {
                time: combatant.time_since_last_move,
            };
            let MeleeAttackFnOutput {
                melee_attack: output_melee_attack,
            } = melee_attack_fn.call(input);
            *melee_attack = output_melee_attack;
            if melee_attack.active {
                hitbox_collision_groups.filters |= GameCollisionGroup::PLAYER.into();
            } else {
                hitbox_collision_groups.filters -= GameCollisionGroup::PLAYER.into();
            }
        } else {
            *melee_attack = default();
            hitbox_collision_groups.filters -= GameCollisionGroup::PLAYER.into();
        }

        if let Some(attack_fn) = &event.move_.projectile_attack_fn {
            let input = ProjectileAttackFnInput {
                _time: combatant.time_since_last_move,
                spawner: entity,
            };
            let ProjectileAttackFnOutput { spawn_events } = attack_fn.call(input);
            spawn_event_writer.send_batch(spawn_events);
        }
    }
    Ok(())
}

#[sysfail(log(level = "error"))]
pub(crate) fn execute_choreography(
    time: Res<Time>,
    mut enemies: Query<(
        Entity,
        &mut Enemy,
        &ConditionTracker,
        &Transform,
        &CurrentMoveMetadata,
    )>,
    mut init_move_event_writer: EventWriter<ReadMoveMetadataEvent>,
    mut execute_move_event_writer: EventWriter<ExecuteMoveFunctionsEvent>,
) -> Result<()> {
    for (entity, mut enemy, condition_tracker, transform, move_metadata) in &mut enemies.iter_mut()
    {
        enemy.update_timers(time.delta_seconds());
        let Some(current) = enemy.current else { continue; };

        let (move_duration, choreography_length) = {
            let moves = &enemy.current_choreography().unwrap().moves;
            let move_ = &moves[current.move_];
            (move_.metadata.duration.clone(), moves.len())
        };

        let time_for_next_move = match move_duration {
            MoveDuration::Fixed(time) => enemy.time_since_last_move >= time,
            MoveDuration::Animation => {
                enemy.time_since_last_animation >= move_metadata.animation_duration.context("MoveDuration::Animation was specified, but no animation duration was found. Did you forget to set an animation?")?
            }
            MoveDuration::Instant => true,
            MoveDuration::While(condition) => !condition_tracker.fulfilled(&condition),
            MoveDuration::Until(condition) => condition_tracker.fulfilled(&condition),
        };
        if time_for_next_move {
            enemy.time_since_last_move = 0.0;
            let was_last_move = current.move_ + 1 >= choreography_length;
            if was_last_move {
                enemy.last_choreography = Some(current.choreography);
                enemy.current = None;
            } else {
                enemy.current = Some(CurrentMove {
                    choreography: current.choreography,
                    move_: current.move_ + 1,
                    start_transform: *transform,
                });

                let next_move = &enemy.current_choreography().unwrap().moves[current.move_ + 1];
                init_move_event_writer.send(ReadMoveMetadataEvent {
                    source: entity,
                    move_: next_move.metadata.clone(),
                });
                execute_move_event_writer.send(ExecuteMoveFunctionsEvent {
                    source: entity,
                    move_: next_move.functions.clone(),
                    duration: next_move.metadata.duration.clone(),
                });
            }
        } else {
            let current_move = &enemy.current_move().unwrap();
            execute_move_event_writer.send(ExecuteMoveFunctionsEvent {
                source: entity,
                move_: current_move.functions.clone(),
                duration: current_move.metadata.duration.clone(),
            });
        }
    }
    Ok(())
}
