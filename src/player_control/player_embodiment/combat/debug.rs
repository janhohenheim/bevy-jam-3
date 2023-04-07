use crate::player_control::player_embodiment::combat::PlayerCombatState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub fn display_combat_state(player: Query<(&PlayerCombatState,)>, mut egui_contexts: EguiContexts) {
    for (combat_state,) in player.iter() {
        egui::Window::new("Player".to_string()).show(egui_contexts.ctx_mut(), |ui| {
            ui.heading("Combat State");
            ui.label(format!("Kind: {:?}", combat_state.kind));
            ui.label(format!("Buffer: {:?}", combat_state.buffer));
            ui.label(format!("Commitment: {:?}", combat_state.commitment));
            ui.label(format!("Time in state: {:.3}", combat_state.time_in_state));
            ui.label(format!(
                "Started animation: {}",
                combat_state.started_animation
            ));
        });
    }
}
