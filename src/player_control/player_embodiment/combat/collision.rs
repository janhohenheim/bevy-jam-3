use crate::combat::collision::PlayerHitEvent;
use crate::player_control::player_embodiment::Player;
use anyhow::Result;
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;

#[sysfail(log(level = "error"))]
pub fn handle_player_being_hit(
    mut hit_events: EventReader<PlayerHitEvent>,
    players: Query<(&Transform,), With<Player>>,
) -> Result<()> {
    for event in hit_events.iter() {
        for (transform,) in players.iter() {
            let angle = transform.forward().angle_between(event.normal);
            info!(
                "Player hit by {} at angle: {}, i.e. normal {}",
                event.attack.name,
                angle.to_degrees(),
                event.normal
            );
        }
    }
    Ok(())
}
