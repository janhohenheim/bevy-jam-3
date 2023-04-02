use crate::combat::ConditionTracker;
use crate::player_control::player_embodiment::Player;
use bevy::prelude::*;

pub fn update_condition_tracker(
    mut combatants: Query<(&mut ConditionTracker, &Transform), Without<Player>>,
    player: Query<&Transform, With<Player>>,
) {
    for (mut condition_tracker, combatant_transform) in combatants.iter_mut() {
        for player_transform in player.iter() {
            let distance_squared =
                (player_transform.translation - combatant_transform.translation).length_squared();
            condition_tracker.player_distance_squared = distance_squared;
        }
    }
}
