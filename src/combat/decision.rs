use crate::combat::components::*;
use anyhow::Result;
use bevy::prelude::*;
use bevy_mod_sysfail::sysfail;
use rand::prelude::*;

#[sysfail(log(level = "error"))]
pub fn decide_choreography(
    mut combatant: Query<(Entity, &mut Combatant)>,
    mut move_event_writer: EventWriter<MoveEvent>,
) -> Result<()> {
    for (entity, mut combatant) in combatant
        .iter_mut()
        .filter(|(_, combatant)| combatant.is_ready_for_next_choreography())
    {
        let next_choreography_index = choose_next_choreography(&combatant)?;
        combatant.current = Some(MoveIndex {
            choreography: next_choreography_index,
            move_: 0,
        });
        let next_move = &combatant.choreographies[next_choreography_index].moves[0];
        move_event_writer.send(MoveEvent {
            source: entity,
            move_: next_move.clone(),
        });
    }
    Ok(())
}

fn choose_next_choreography(combatant: &Combatant) -> Result<usize> {
    let mut rng = thread_rng();
    let choices = &combatant.tendencies;
    let next_choreography_index = choices
        .choose_weighted(&mut rng, |item| item.weight)?
        .choreography;
    Ok(next_choreography_index)
}
