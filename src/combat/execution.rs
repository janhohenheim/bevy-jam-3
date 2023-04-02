use crate::combat::components::*;
use crate::level_instantiation::spawning::AnimationEntityLink;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_mod_sysfail::sysfail;
use std::time::Duration;

#[sysfail(log(level = "error"))]
pub fn execute_move(
    mut move_events: EventReader<MoveEvent>,
    mut animation_player: Query<&mut AnimationPlayer>,
    mut combatant: Query<(&AnimationEntityLink, &mut CombatantState)>,
) -> Result<()> {
    for event in move_events.iter() {
        let move_ = &event.move_;
        let (animation_entity_link, mut combatant_state) = combatant.get_mut(event.source)?;
        let mut animation_player = animation_player
            .get_mut(**animation_entity_link)
            .context("animation_entity_link held entity without animation player")?;

        animation_player
            .play_with_transition(move_.animation.clone(), Duration::from_secs_f32(0.2));
        *combatant_state = move_.state;
    }
    Ok(())
}

pub fn execute_choreography(
    time: Res<Time>,
    mut combatants: Query<(Entity, &mut Combatant)>,
    mut move_event_writer: EventWriter<MoveEvent>,
) {
    for (entity, mut combatant) in &mut combatants.iter_mut() {
        if let Some(current) = combatant.current {
            let (move_duration, choreography_length) = {
                let moves = &combatant.choreographies[current.choreography].moves;
                let move_ = &moves[current.move_];
                (move_.duration, moves.len())
            };

            if combatant.time_since_last_move >= move_duration {
                combatant.time_since_last_move = 0.0;
                let was_last_move = current.move_ + 1 >= choreography_length;
                if was_last_move {
                    combatant.current = None;
                } else {
                    combatant.current = Some(MoveIndex {
                        choreography: current.choreography,
                        move_: current.move_ + 1,
                    });

                    let next_move =
                        &combatant.choreographies[current.choreography].moves[current.move_ + 1];
                    move_event_writer.send(MoveEvent {
                        source: entity,
                        move_: next_move.clone(),
                    });
                }
            } else {
                combatant.time_since_last_move += time.delta_seconds();
            }
        }
    }
}