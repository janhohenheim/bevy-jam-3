use crate::combat::{Combatant, CombatantState, ConditionTracker};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub fn display_combatants(
    combatant: Query<(&Name, &Combatant, &CombatantState, &ConditionTracker)>,
    mut egui_contexts: EguiContexts,
) {
    for (name, combatant, combatant_state, condition_tracker) in combatant.iter() {
        egui::Window::new(format!("Combatant: {name}")).show(egui_contexts.ctx_mut(), |ui| {
            if let Some(current) = combatant.current {
                let choreography = &combatant.choreographies[current.choreography];
                ui.label(format!("Current choreography: {}", choreography.name));
                ui.label(format!(
                    "Current move index: {}/{}",
                    current.move_,
                    choreography.moves.len() - 1
                ));
            } else {
                ui.label("Current choreography: None");
                ui.label("Current move: None");
            }
            ui.label(format!("State: {combatant_state:?}"));
            ui.label(format!(
                "Time since last move: {}",
                combatant.time_since_last_move
            ));
            ui.label(format!("Condition tracker: {condition_tracker:?}"));
        });
    }
}
