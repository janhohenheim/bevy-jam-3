use crate::combat::components::*;
use anyhow::Result;
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;
use rand::prelude::*;

#[sysfail(log(level = "error"))]
pub fn decide_choreography(
    mut combatant: Query<(Entity, &mut Combatant, &ConditionTracker, &Transform)>,
    mut move_event_writer: EventWriter<MoveEvent>,
) -> Result<()> {
    for (entity, mut combatant, condition_tracker, transform) in combatant
        .iter_mut()
        .filter(|(_, combatant, _, _)| combatant.is_ready_for_next_choreography())
    {
        let next_choreography_index = get_chained_choreography(&combatant)
            .map(Ok)
            .unwrap_or_else(|| choose_next_choreography(&combatant, condition_tracker))?;
        combatant.current = Some(CurrentMove {
            choreography: next_choreography_index,
            move_: 0,
            start_transform: *transform,
        });
        let next_move = &combatant.choreographies[next_choreography_index].moves[0];
        move_event_writer.send(MoveEvent {
            source: entity,
            move_: next_move.clone(),
        });
    }
    Ok(())
}

fn get_chained_choreography(combatant: &Combatant) -> Option<usize> {
    combatant
        .last_choreography
        .and_then(|index| combatant.chained_choreographies.get(&index))
        .copied()
}

fn choose_next_choreography(
    combatant: &Combatant,
    condition_tracker: &ConditionTracker,
) -> Result<usize> {
    let mut rng = thread_rng();
    let choices: Vec<_> = combatant
        .tendencies
        .iter()
        .filter(|tendency| condition_tracker.all(&tendency.conditions))
        .collect();
    let next_choreography_index = choices
        .choose_weighted(&mut rng, |item| item.weight)?
        .choreography;
    Ok(next_choreography_index)
}
