use crate::combat::{Combatant, CombatantState, ConditionTracker};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub fn display_combatants(
    combatant: Query<(&Name, &Combatant, &CombatantState, &ConditionTracker)>,
    mut egui_contexts: EguiContexts,
) {
    for (name, combatant, combatant_state, condition_tracker) in combatant.iter() {
        egui::Window::new(format!("Combatant: {name}")).show(egui_contexts.ctx_mut(), |ui| {
            ui.heading("Choreography");
            if let Some(current) = combatant.current {
                let choreography = &combatant.choreographies[current.choreography];
                let move_ = &choreography.moves[current.move_];
                ui.label(format!("Name: {}", choreography.name));
                ui.label(format!("Move: {:?}", move_.name));
                ui.label(format!(
                    "Move index: {}/{}",
                    current.move_ + 1,
                    choreography.moves.len()
                ));
            } else {
                ui.label("Name: None");
                ui.label("Move: None");
            }
            ui.heading("Condition Tracker");
            ui.label(format!(
                "Player direction: {}",
                condition_tracker.player_direction.format()
            ));
            ui.label("Line of sight path:");
            for direction in &condition_tracker.line_of_sight_path {
                ui.label(format!("• {}", direction.format()));
            }
            ui.label(format!(
                "Has line of sight: {}",
                condition_tracker.has_line_of_sight
            ));
            ui.label(format!("Is active: {}", condition_tracker.active));

            ui.heading("Misc");
            ui.label(format!("State: {combatant_state:?}"));
            ui.label(format!(
                "Time since last move: {:.3}",
                combatant.time_since_last_move
            ));
        });
    }
}

trait Vec3Ext: Copy {
    fn format(self) -> String;
}

impl Vec3Ext for Vec3 {
    fn format(self) -> String {
        format!("[{:.3}, {:.3}, {:.3}]", self.x, self.y, self.z)
    }
}
