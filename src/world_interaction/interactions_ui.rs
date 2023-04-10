use crate::player_control::actions::PlayerAction;
use crate::player_control::camera::IngameCamera;
use crate::util::criteria::is_frozen;
use crate::util::trait_extension::F32Ext;
use crate::world_interaction::room::{CurrentRoom, Exit, LeaveRoomEvent};
use crate::GameState;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContexts};
use bevy_mod_sysfail::macros::*;
use leafwing_input_manager::prelude::ActionState;

pub(crate) fn interactions_ui_plugin(app: &mut App) {
    app.add_system(
        display_interaction_prompt
            .run_if(not(is_frozen))
            .in_set(OnUpdate(GameState::Playing)),
    );
}

fn is_facing_target(camera_transform: Transform, target_transform: Transform) -> bool {
    let look_direction = camera_transform.forward();
    let target_direction = target_transform.translation - camera_transform.translation;
    let angle = look_direction.angle_between(target_direction);
    angle.to_degrees() < 45.0
}

#[sysfail(log(level = "error"))]
fn display_interaction_prompt(
    mut egui_contexts: EguiContexts,
    players: Query<
        (&Transform, &ActionState<PlayerAction>),
        (Without<IngameCamera>, Without<Exit>),
    >,
    cameras: Query<&Transform, (With<IngameCamera>, Without<Exit>)>,
    primary_windows: Query<&Window, With<PrimaryWindow>>,
    exits: Query<&Transform, With<Exit>>,
    current_room: Res<CurrentRoom>,
    mut leave_room_events: EventWriter<LeaveRoomEvent>,
) -> Result<()> {
    for (player_transform, actions) in players.iter() {
        for exit_transform in exits.iter() {
            for camera_transform in cameras.iter() {
                let is_near_exit = (player_transform.translation - exit_transform.translation)
                    .length_squared()
                    < 4.0.squared();
                let should_display =
                    is_facing_target(*camera_transform, *exit_transform) && is_near_exit;
                if !should_display {
                    continue;
                }
                let window = primary_windows
                    .get_single()
                    .context("Failed to get primary window")?;
                let message = if current_room.cleared {
                    "E: Choose side effects"
                } else {
                    "Kill all enemies before leaving"
                };
                egui::Window::new("Interaction")
                    .collapsible(false)
                    .title_bar(false)
                    .auto_sized()
                    .fixed_pos(egui::Pos2::new(window.width() / 2., window.height() / 2.))
                    .show(egui_contexts.ctx_mut(), |ui| {
                        ui.label(message);
                    });
                if current_room.cleared && actions.just_pressed(PlayerAction::Interact) {
                    leave_room_events.send(LeaveRoomEvent);
                    info!("Interacting with exit");
                }
            }
        }
    }
    Ok(())
}
