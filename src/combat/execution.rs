use crate::combat::components::*;
use crate::level_instantiation::spawning::objects::GameCollisionGroup;
use crate::level_instantiation::spawning::AnimationEntityLink;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;
use bevy_rapier3d::prelude::*;
use std::time::Duration;

#[sysfail(log(level = "error"))]
pub fn init_move(
    mut move_events: EventReader<InitMoveEvent>,
    mut animation_player: Query<&mut AnimationPlayer>,
    mut combatant: Query<(
        &AnimationEntityLink,
        &mut CombatantState,
        &mut MoveMetadata,
        &ConditionTracker,
        &Transform,
    )>,
) -> Result<()> {
    for event in move_events.iter() {
        let move_ = &event.move_;
        let (
            animation_entity_link,
            mut combatant_state,
            mut move_metadata,
            condition_tracker,
            transform,
        ) = combatant.get_mut(event.source)?;
        let mut animation_player = animation_player
            .get_mut(**animation_entity_link)
            .context("animation_entity_link held entity without animation player")?;

        if let Some(animation) = &move_.animation {
            animation_player
                .play_with_transition(animation.clone(), Duration::from_secs_f32(0.2))
                .repeat();
        }
        *combatant_state = move_.state;
        *move_metadata = MoveMetadata {
            start_transform: *transform,
            start_player_direction: condition_tracker.player_direction,
        };
    }
    Ok(())
}

#[sysfail(log(level = "error"))]
pub fn execute_move(
    mut combatants: Query<
        (
            &Combatant,
            &ConditionTracker,
            &mut Transform,
            &ReadMassProperties,
            &mut ExternalForce,
            &MoveMetadata,
            &MeleeAttackLink,
        ),
        Without<MeleeAttack>,
    >,
    mut melee_attacks: Query<(
        &mut MeleeAttack,
        &mut CollisionGroups,
        &mut Transform,
        &mut Collider,
    )>,
    mut move_events: EventReader<ExecuteMoveEvent>,
) -> Result<()> {
    for event in move_events.iter() {
        let entity = event.source;
        let (
            combatant,
            condition_tracker,
            mut transform,
            mass,
            mut force,
            move_metadata,
            melee_attack_link,
        ) = combatants.get_mut(entity)?;
        if let Some(force_fn) = &event.move_.force_fn {
            let input = ForceFnInput {
                time: combatant.time_since_last_move,
                transform: *transform,
                start_transform: move_metadata.start_transform,
                player_direction: condition_tracker.player_direction,
                start_player_direction: move_metadata.start_player_direction,
                has_line_of_sight: condition_tracker.has_line_of_sight,
                line_of_sight_path: condition_tracker.line_of_sight_path.clone(),
                mass: mass.0.mass,
            };
            let ForceFnOutput {
                force: output_force,
                rotation,
            } = force_fn.call(input);
            *force = output_force;
            if let Some(rotation) = rotation {
                transform.rotation = rotation;
            }
        }

        if let Some(melee_attack_fn) = &event.move_.melee_attack_fn {
            let (
                mut melee_attack,
                mut attack_collision_groups,
                mut attack_transform,
                mut attack_collider,
            ) = melee_attacks.get_mut(melee_attack_link.0)?;
            attack_collision_groups.filters |= GameCollisionGroup::PLAYER.into();
            let input = MeleeAttackFnInput {
                time: combatant.time_since_last_move,
            };
            let MeleeAttackFnOutput {
                collider: output_collider,
                transform: output_transform,
                damage,
                knockback,
            } = melee_attack_fn.call(input);
            *attack_transform = output_transform;
            *attack_collider = output_collider;
            melee_attack.damage = damage;
            melee_attack.knockback = knockback;
        } else {
            let mut attack_collision_groups = melee_attacks.get_mut(melee_attack_link.0)?.1;
            attack_collision_groups.filters -= GameCollisionGroup::PLAYER.into();
        }
    }
    Ok(())
}

pub fn execute_choreography(
    time: Res<Time>,
    mut combatants: Query<(Entity, &mut Combatant, &ConditionTracker, &Transform)>,
    mut init_move_event_writer: EventWriter<InitMoveEvent>,
    mut execute_move_event_writer: EventWriter<ExecuteMoveEvent>,
) {
    for (entity, mut combatant, condition_tracker, transform) in &mut combatants.iter_mut() {
        combatant.time_since_last_move += time.delta_seconds();
        let Some(current) = combatant.current else { continue; };

        let (move_duration, choreography_length) = {
            let moves = &combatant.choreographies[current.choreography].moves;
            let move_ = &moves[current.move_];
            (move_.init.duration.clone(), moves.len())
        };

        let time_for_next_move = match move_duration {
            MoveDuration::Fixed(time) => combatant.time_since_last_move >= time,
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
                });
            }
        } else {
            let current_move = &combatant.choreographies[current.choreography].moves[current.move_];
            execute_move_event_writer.send(ExecuteMoveEvent {
                source: entity,
                move_: current_move.execute.clone(),
            });
        }
    }
}
