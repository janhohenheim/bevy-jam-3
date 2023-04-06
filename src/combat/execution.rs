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
pub fn init_move(
    mut move_events: EventReader<InitMoveEvent>,
    mut animation_player: Query<&mut AnimationPlayer>,
    mut combatant: Query<(
        &AnimationEntityLink,
        &mut Combatant,
        &mut CombatantState,
        &mut MoveMetadata,
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
        ) = combatant.get_mut(event.source)?;
        let mut animation_player = animation_player
            .get_mut(**animation_entity_link)
            .context("animation_entity_link held entity without animation player")?;

        if let Some(animation) = &move_.animation {
            combatant.time_since_last_animation = 0.0;
            animation_player.play_with_transition(animation.clone(), Duration::from_secs_f32(0.2));
            if move_.duration != MoveDuration::Animation {
                animation_player.repeat();
            }
        }

        let animation_duration = move_
            .animation
            .as_ref()
            .and_then(|animation| animations.get(animation))
            .map(|animation| animation.duration())
            .or(move_metadata.animation_duration);

        *combatant_state = move_.state;
        *move_metadata = MoveMetadata {
            start_transform: *transform,
            start_player_direction: condition_tracker.player_direction,
            animation_duration,
        };
    }
    Ok(())
}

#[sysfail(log(level = "error"))]
pub fn execute_move(
    time: Res<Time>,
    mut combatants: Query<(
        &Combatant,
        &ConditionTracker,
        &mut Transform,
        &ReadMassProperties,
        &mut ExternalForce,
        &mut ExternalImpulse,
        &MoveMetadata,
        &Velocity,
        &ParentToHitboxLink,
    )>,
    mut melee_attacks: Query<(&mut AttackHitbox, &mut CollisionGroups)>,
    mut move_events: EventReader<ExecuteMoveEvent>,
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
        ) = combatants.get_mut(entity)?;
        if let Some(motion_fn) = &event.move_.motion_fn {
            let duration = match event.duration {
                MoveDuration::Animation => move_metadata.animation_duration,
                MoveDuration::Fixed(duration) => Some(duration),
                _ => None,
            };
            let input = MotionFnInput {
                time: combatant.time_since_last_move,
                duration,
                transform: *transform,
                start_transform: move_metadata.start_transform,
                player_direction: condition_tracker.player_direction,
                start_player_direction: move_metadata.start_player_direction,
                has_line_of_sight: condition_tracker.has_line_of_sight,
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
                time: combatant.time_since_last_move,
                spawner: entity,
            };
            let ProjectileAttackFnOutput { spawn_events } = attack_fn.call(input);
            spawn_event_writer.send_batch(spawn_events);
        }
    }
    Ok(())
}

#[sysfail(log(level = "error"))]
pub fn execute_choreography(
    time: Res<Time>,
    mut combatants: Query<(
        Entity,
        &mut Combatant,
        &ConditionTracker,
        &Transform,
        &MoveMetadata,
    )>,
    mut init_move_event_writer: EventWriter<InitMoveEvent>,
    mut execute_move_event_writer: EventWriter<ExecuteMoveEvent>,
) -> Result<()> {
    for (entity, mut combatant, condition_tracker, transform, move_metadata) in
        &mut combatants.iter_mut()
    {
        combatant.time_since_last_move += time.delta_seconds();
        combatant.time_since_last_animation += time.delta_seconds();
        let Some(current) = combatant.current else { continue; };

        let (move_duration, choreography_length) = {
            let moves = &combatant.choreographies[current.choreography].moves;
            let move_ = &moves[current.move_];
            (move_.init.duration.clone(), moves.len())
        };

        let time_for_next_move = match move_duration {
            MoveDuration::Fixed(time) => combatant.time_since_last_move >= time,
            MoveDuration::Animation => {
                combatant.time_since_last_animation >= move_metadata.animation_duration.context("MoveDuration::Animation was specified, but no animation duration was found. Did you forget to set an animation?")?
            }
            MoveDuration::Instant => true,
            MoveDuration::While(condition) => !condition_tracker.fulfilled(&condition),
            MoveDuration::Until(condition) => condition_tracker.fulfilled(&condition),
        };
        if time_for_next_move {
            combatant.time_since_last_move = 0.0;
            let was_last_move = current.move_ + 1 >= choreography_length;
            if was_last_move {
                combatant.last_choreography = Some(current.choreography);
                combatant.current = None;
            } else {
                combatant.current = Some(CurrentMove {
                    choreography: current.choreography,
                    move_: current.move_ + 1,
                    start_transform: *transform,
                });

                let next_move =
                    &combatant.choreographies[current.choreography].moves[current.move_ + 1];
                init_move_event_writer.send(InitMoveEvent {
                    source: entity,
                    move_: next_move.init.clone(),
                });
                execute_move_event_writer.send(ExecuteMoveEvent {
                    source: entity,
                    move_: next_move.execute.clone(),
                    duration: next_move.init.duration.clone(),
                });
            }
        } else {
            let current_move = &combatant.choreographies[current.choreography].moves[current.move_];
            execute_move_event_writer.send(ExecuteMoveEvent {
                source: entity,
                move_: current_move.execute.clone(),
                duration: current_move.init.duration.clone(),
            });
        }
    }
    Ok(())
}
