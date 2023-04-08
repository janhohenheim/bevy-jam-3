use crate::combat::Constitution;
use crate::player_control::player_embodiment::combat::{BlockHistory, PlayerCombatState};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub fn display_combat_state(
    player: Query<(&PlayerCombatState, &Constitution, &BlockHistory)>,
    mut egui_contexts: EguiContexts,
) {
    for (combat_state, constitution, block_history) in player.iter() {
        egui::Window::new("Player".to_string()).show(egui_contexts.ctx_mut(), |ui| {
            ui.heading("Combat State");
            ui.label(format!("Kind: {:?}", combat_state.kind));
            ui.label(format!("Buffer: {:?}", combat_state.buffer));
            ui.label(format!("Commitment: {:?}", combat_state.commitment));
            ui.label(format!("Time in state: {:.3}", combat_state.time_in_state));
            ui.label(format!(
                "Time since hit: {:.3}",
                combat_state.time_since_hit
            ));
            ui.label(format!(
                "Time since sprint: {:.3}",
                combat_state.time_since_sprint
            ));
            ui.label(format!(
                "Started animation: {}",
                combat_state.started_animation
            ));

            ui.heading("Constitution");
            ui.label(format!("Health: {:.3}", constitution.health()));
            ui.label(format!("Posture: {:.3}", constitution.posture()));
            ui.label(format!("Is dead: {}", constitution.is_dead()));
            ui.label(format!(
                "Is posture broken: {}",
                constitution.is_posture_broken()
            ));

            ui.heading("Block History");
            ui.label(format!(
                "Younger than 0.5s: {}",
                block_history.count_younger_than(0.5)
            ));
            ui.label(format!(
                "Younger than 1.0s: {}",
                block_history.count_younger_than(1.0)
            ));
            ui.label(format!(
                "Younger than 1.5s: {}",
                block_history.count_younger_than(1.5)
            ));
            ui.label(format!(
                "Younger than 2.0s: {}",
                block_history.count_younger_than(2.0)
            ));
            ui.label(format!(
                "Current Deflect Streak: {}",
                block_history.current_deflect_streak()
            ));
        });
    }
}
