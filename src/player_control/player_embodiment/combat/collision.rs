use crate::combat::collision::PlayerHitEvent;
use crate::combat::{Attack, Combatant, Constitution};
use crate::player_control::player_embodiment::combat::{PlayerCombatKind, PlayerCombatState};
use crate::player_control::player_embodiment::Player;
use anyhow::Result;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize, FromReflec, Default)]
#[reflect(Serialize, Deserialize)]
pub struct PlayerHurtEvent {
    pub(crate) attack: Attack,
    pub(crate) extent: PlayerHurtExtent,
}

#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize, FromReflect, Default)]
#[reflect(Serialize, Deserialize)]
pub enum PlayerHurtExtent {
    #[default]
    Full,
    Health,
    Posture,
}

#[sysfail(log(level = "error"))]
pub fn handle_player_being_hit(
    mut hit_events: EventReader<PlayerHitEvent>,
    mut players: Query<
        (&Transform, &PlayerCombatState, &mut Constitution),
        (With<Player>, Without<Combatant>),
    >,
    mut enemies: Query<(&mut Constitution,), (With<Combatant>, Without<Player>)>,
    mut hurt_events: EventWriter<PlayerHurtEvent>,
) -> Result<()> {
    for event in hit_events.iter() {
        for (transform, combat_state, mut constitution) in players.iter_mut() {
            if combat_state.kind == PlayerCombatKind::Block {
                let angle = transform
                    .forward()
                    .xz()
                    .angle_between(event.target_to_contact.xz())
                    .to_degrees();
                if angle.abs() > 100.0 {
                    constitution.take_full_damage(&event.attack);
                } else if combat_state.time_in_state < 0.2 {
                    let (mut enemy_constitution,) = enemies.get_mut(event.source)?;
                    enemy_constitution.take_posture_damage(&event.attack);
                } else {
                    constitution.take_posture_damage(&event.attack);
                }
            } else {
                constitution.take_full_damage(&event.attack);
            }
        }
    }
    Ok(())
}
