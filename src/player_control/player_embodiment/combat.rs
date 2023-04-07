use crate::player_control::player_embodiment::PlayerAction;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub fn attack(players: Query<&ActionState<PlayerAction>>) {
    for actions in players.iter() {
        if actions.just_released(PlayerAction::Attack) {
            println!("Attack!");
        }
    }
}
