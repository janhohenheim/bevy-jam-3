use crate::combat::collision::detection::EnemyHitEvent;
use crate::combat::{Combatant, CombatantState, Constitution};
use anyhow::Result;
use bevy::math::Vec3Swizzles;
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

        let angle = transform
            .forward()
            .xz()
            .angle_between(event.target_to_contact.xz());
        info!(
            "Enemy hit by {} at angle: {}",
            event.attack.name,
            angle.to_degrees(),
        );
        match combatant.current_move() {
            Some(move_) => match move_.init.state {
                CombatantState::Deathblow => {
                    constitution.die();
                }
                CombatantState::Vulnerable => {
                    constitution.take_full_damage(&event.attack);
                }
                CombatantState::OnGuard => {
                    constitution.take_posture_damage(&event.attack);
                }
                CombatantState::HyperArmor => {
                    constitution.take_health_damage(&event.attack);
                }
            },
            None => {}
        }
    }
    Ok(())
}
