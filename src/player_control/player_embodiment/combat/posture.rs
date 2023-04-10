use crate::combat::Constitution;
use crate::file_system_interaction::level_serialization::WorldLoadRequest;
use crate::movement::general_movement::Walking;
use crate::player_control::actions::ActionsFrozen;
use crate::player_control::player_embodiment::combat::{
    AttackCommitment, PlayerCombatKind, PlayerCombatState,
};
use crate::player_control::player_embodiment::Player;
use crate::world_interaction::room::CurrentRoom;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub(crate) fn update_posture(
    time: Res<Time>,
    mut player: Query<(&mut PlayerCombatState, &mut Constitution, &Walking)>,
) {
    for (mut combat_state, mut constitution, walking) in player.iter_mut() {
        if constitution.is_posture_broken() {
            combat_state.force_use_next_kind(PlayerCombatKind::PostureBroken);
            combat_state.commitment = AttackCommitment::Committed;
            constitution.mark_broken_posture_as_handled();
        }
        let posture_recovery_time = if walking.sprinting {
            None
        } else {
            match combat_state.kind {
                PlayerCombatKind::Idle => Some(0.7),
                PlayerCombatKind::Attack(_) => None,
                PlayerCombatKind::Block => Some(0.7),
                PlayerCombatKind::Deflected => None,
                PlayerCombatKind::PostureBroken => None,
                PlayerCombatKind::Hurt => None,
            }
        };
        let Some(posture_recovery_time) = posture_recovery_time else {
            continue;
        };

        let time_since_hurt_or_block = 1.8;
        if combat_state.time_in_state > posture_recovery_time
            && combat_state.time_since_sprint > posture_recovery_time
            && combat_state.time_since_hurt_or_block > time_since_hurt_or_block
        {
            let factor = if combat_state.kind == PlayerCombatKind::Block {
                2.0
            } else {
                1.0
            };
            constitution.recover_posture(time.delta_seconds() * factor);
        }
    }
}

pub(crate) fn handle_death(
    mut player: Query<&Constitution, With<Player>>,
    mut loader: EventWriter<WorldLoadRequest>,
    mut pause: Local<bool>,
    mut actions_frozen: ResMut<ActionsFrozen>,
    mut egui_contexts: EguiContexts,
    current_room: Res<CurrentRoom>,
) {
    let room_number = current_room.number;
    for constitution in player.iter_mut() {
        if constitution.is_dead() {
            if !*pause {
                *pause = true;
                actions_frozen.freeze();
            }
        }
    }
    if *pause {
        egui::CentralPanel::default()
            .frame(egui::Frame {
                fill: egui::Color32::from_black_alpha(240),
                ..default()
            })
            .show(egui_contexts.ctx_mut(), |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.visuals_mut().override_text_color = Some(egui::Color32::from_gray(240));
                    ui.add_space(100.0);
                    ui.heading("You died");
                    ui.separator();
                    ui.vertical_centered_justified(|ui| {
                        ui.label(format!("You beat {room_number} rooms!"));
                        ui.label("Wanna try again?");
                        if ui.button("Heck yeah!").clicked() {
                            *pause = false;
                            actions_frozen.unfreeze();
                            loader.send(WorldLoadRequest {
                                filename: "intro_room".to_string(),
                            });
                        }
                    });
                });
            });
    }
}
