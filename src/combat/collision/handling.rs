use crate::combat::collision::detection::EnemyHitEvent;
use crate::combat::Combatant;
use anyhow::Result;
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;

#[sysfail(log(level = "error"))]
pub fn handle_enemy_being_hit(
    mut hit_events: EventReader<EnemyHitEvent>,
    mut combatants: Query<&mut Combatant>,
) -> Result<()> {
    for event in hit_events.iter() {
        let mut _combatant = combatants
            .get_mut(event.target)
            .expect("Failed to get combatant from hit event");
        info!("Enemy hit by: {:?}", event.attack);
    }
    Ok(())
}
