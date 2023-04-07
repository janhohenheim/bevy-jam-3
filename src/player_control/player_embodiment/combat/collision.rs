use crate::combat::collision::PlayerHitEvent;
use anyhow::Result;
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;

#[sysfail(log(level = "error"))]
pub fn handle_player_being_hit(mut hit_events: EventReader<PlayerHitEvent>) -> Result<()> {
    for event in hit_events.iter() {
        info!("Player hit by: {:?}", event.attack);
    }
    Ok(())
}
