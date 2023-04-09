use crate::combat::components::*;
use anyhow::Result;
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;
use rand::prelude::*;

#[sysfail(log(level = "error"))]
pub(crate) fn decide_choreography(
    mut combatant: Query<(Entity, &mut Enemy, &ConditionTracker, &Transform)>,
    mut init_move_event_writer: EventWriter<ReadMoveMetadataEvent>,
    mut execute_move_event_writer: EventWriter<ExecuteMoveFunctionsEvent>,
) -> Result<()> {
    for (entity, mut combatant, condition_tracker, transform) in combatant
        .iter_mut()
        .filter(|(_, combatant, _, _)| combatant.is_ready_for_next_choreography())
    {
        let next_choreography_index = choose_next_choreography(&combatant, condition_tracker)?;
        combatant.forced_choreography = None;
        if let Some(current) = combatant.current {
            combatant.last_choreography = Some(current.choreography);
        }
        combatant.current = Some(CurrentMove {
            choreography: next_choreography_index,
            move_: 0,
            start_transform: *transform,
        });
        let next_move = &combatant.choreographies[next_choreography_index].moves[0];
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
    Ok(())
}

fn choose_next_choreography(
    combatant: &Enemy,
    condition_tracker: &ConditionTracker,
) -> Result<usize> {
    combatant
        .forced_choreography
        .or(get_chained_choreography(&combatant))
        .map(Ok)
        .unwrap_or_else(|| roll_next_choreography(&combatant, condition_tracker))
}

fn get_chained_choreography(enemy: &Enemy) -> Option<usize> {
    enemy
        .last_choreography
        .and_then(|index| enemy.chained_choreographies.get(&index))
        .copied()
}

fn roll_next_choreography(enemy: &Enemy, condition_tracker: &ConditionTracker) -> Result<usize> {
    let mut rng = thread_rng();
    let choices: Vec<_> = enemy
        .tendencies
        .iter()
        .filter(|tendency| condition_tracker.fulfilled(&tendency.condition))
        .collect();
    let next_choreography_index = choices
        .choose_weighted(&mut rng, |item| item.weight)?
        .choreography;
    Ok(next_choreography_index)
}
