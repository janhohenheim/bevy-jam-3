use crate::combat::collision::detection::EnemyHitEvent;
use crate::combat::{Combatant, CombatantState, Constitution};
use anyhow::Result;
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;

#[sysfail(log(level = "error"))]
pub fn handle_enemy_being_hit(
    mut hit_events: EventReader<EnemyHitEvent>,
    mut combatants: Query<(&mut Combatant, &mut Constitution, &Transform)>,
) -> Result<()> {
    for event in hit_events.iter() {
        let (mut combatant, mut constitution, transform) = combatants
            .get_mut(event.target)
            .expect("Failed to get combatant from hit event");

        let angle = transform.forward().angle_between(event.normal);
        info!(
            "Enemy hit by {} at angle: {}, i.e. normal {}",
            event.attack.name,
            angle.to_degrees(),
            event.normal
        );
        match combatant.current_move() {
            Some(move_) => match move_.init.state {
                CombatantState::Deathblow => {
                    constitution.health = 0.0;
                }
                CombatantState::Vulnerable => {
                    constitution.health -= event.attack.damage;
                }
                CombatantState::OnGuard => {
                    constitution.posture += event.attack.damage;
                }
                CombatantState::HyperArmor => {
                    constitution.health -= event.attack.damage;
                }
            },
            None => {}
        }
    }
    Ok(())
}
