use crate::combat::components::*;
use bevy::prelude::*;

pub fn decide_choreography(mut combatant: Query<(&mut Combatant,)>) {
    for (mut combatant,) in combatant
        .iter_mut()
        .filter(|(combatant,)| combatant.current.is_none())
    {
        if combatant.current.is_none() {}
    }
}
