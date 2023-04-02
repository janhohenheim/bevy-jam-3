use crate::level_instantiation::spawning::AnimationEntityLink;
use crate::GameState;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_mod_sysfail::sysfail;
use std::time::Duration;

mod components;
pub use components::*;

pub fn combat_plugin(app: &mut App) {
    app.register_type::<CombatantState>()
        .register_type::<Choreography>()
        .register_type::<Move>()
        .add_event::<MoveEvent>()
        .add_systems(
            (execute_move, execute_current_choreography).in_set(OnUpdate(GameState::Playing)),
        );
}

#[sysfail(log(level = "error"))]
fn execute_move(
    mut move_events: EventReader<MoveEvent>,
    mut animation_player: Query<&mut AnimationPlayer>,
    mut attackers: Query<(&AnimationEntityLink, &mut CombatantState)>,
) -> Result<()> {
    for event in move_events.iter() {
        let move_ = &event.move_;
        let (animation_entity_link, mut combatant_state) = attackers.get_mut(event.source)?;
        let mut animation_player = animation_player
            .get_mut(**animation_entity_link)
            .context("animation_entity_link held entity without animation player")?;

        animation_player
            .play_with_transition(move_.animation.clone(), Duration::from_secs_f32(0.2));
        *combatant_state = move_.state;
    }
    Ok(())
}

fn execute_current_choreography(
    time: Res<Time>,
    mut combatants: Query<(Entity, &mut Combatant)>,
    mut move_event_writer: EventWriter<MoveEvent>,
) {
    for (entity, mut combatant) in &mut combatants.iter_mut() {
        if let Some(current) = combatant.current {
            let (move_duration, choreography_length) = {
                let choreography = &combatant.choreographies[current.choreography];
                let move_ = &choreography[current.move_];
                (move_.duration, choreography.len())
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
                        &combatant.choreographies[current.choreography][current.move_ + 1];
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
