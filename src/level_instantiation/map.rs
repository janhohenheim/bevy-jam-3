use crate::file_system_interaction::level_serialization::{CurrentLevel, WorldLoadRequest};
use crate::level_instantiation::spawning::post_spawn_modification::set_shadows;
use crate::level_instantiation::spawning::GameObject;
use crate::player_control::player_embodiment::Player;
use crate::GameState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use regex::Regex;
use spew::prelude::*;
use std::sync::LazyLock;

pub(crate) fn map_plugin(app: &mut App) {
    app.add_system(
        setup
            .run_if(not(resource_exists::<CurrentLevel>()))
            .in_schedule(OnEnter(GameState::Playing)),
    )
    .add_system(
        show_loading_screen
            .run_if(not(any_with_component::<Player>()))
            .in_set(OnUpdate(GameState::Playing)),
    )
    .add_systems(
        (spawn_enemies, place_player)
            .chain()
            .after(set_shadows)
            .in_set(OnUpdate(GameState::Playing)),
    );
    #[cfg(feature = "wasm")]
    app.add_system(show_wasm_loader.in_set(OnUpdate(GameState::Playing)));
}

fn setup(mut commands: Commands, mut loader: EventWriter<WorldLoadRequest>) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.05,
    });

    loader.send(WorldLoadRequest {
        filename: "intro_room".to_string(),
    });
}

fn place_player(
    mut commands: Commands,
    names: Query<(Entity, &Transform, &Name), (Added<Name>, Without<Player>)>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut spawn_events: EventWriter<SpawnEvent<GameObject, Transform>>,
) {
    for (entity, transform, name) in names.iter() {
        if name.contains("[entrance]") {
            commands.entity(entity).despawn_recursive();
            if let Ok(mut player_transform) = player_query.get_single_mut() {
                *player_transform = transform.clone();
            } else {
                spawn_events.send(
                    SpawnEvent::with_data(GameObject::Player, transform.clone()).delay_frames(2),
                );
            }
        }
    }
}

static ENEMY_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[enemy:\s*(\w+)\]").expect("Failed to compile enemy regex"));

fn spawn_enemies(
    mut commands: Commands,
    names: Query<(Entity, &Transform, &Name), Added<Name>>,
    mut spawn_events: EventWriter<SpawnEvent<GameObject, Transform>>,
) {
    for (entity, transform, name) in names.iter() {
        if let Some(captures) = ENEMY_REGEX.captures(&name.to_lowercase()) {
            commands.entity(entity).despawn_recursive();
            let enemy_name = captures.get(1).unwrap().as_str();
            match enemy_name {
                "dummy" => {
                    let transform = transform.with_scale(Vec3::splat(1.));
                    spawn_events.send(SpawnEvent::with_data(GameObject::Dummy, transform));
                }
                _ => {
                    error!("Tried to spawn invalid enemy type: {}", enemy_name);
                }
            }
        }
    }
}

fn show_loading_screen(mut egui_contexts: EguiContexts) {
    egui::CentralPanel::default().show(egui_contexts.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(100.0);
            ui.heading("Loading");
            ui.label("Spawning level...");
            ui.add_space(10.0);
            #[cfg(feature = "wasm")]
            ui.add_space(40.0); // Spinner from CSS (build/web/styles.css) goes here.
            #[cfg(feature = "wasm")]
            ui.label("This may take a while. Don't worry, your browser did not crash!");
        });
    });
}

#[cfg(feature = "wasm")]
fn show_wasm_loader(player_query: Query<&Player>, mut egui_contexts: EguiContexts) {
    let id = egui::Id::new("loading-screen-shown");
    egui_contexts.ctx_mut().memory_mut(|memory| {
        let memory = &mut memory.data;
        match (memory.get_temp::<()>(id), player_query.iter().next()) {
            (None, None) => {
                loader::show_loader();
                memory.insert_temp(id, ());
            }
            (Some(_), Some(_)) => {
                loader::hide_loader();
                memory.remove::<()>(id);
            }
            _ => {}
        }
    });
}

#[cfg(feature = "wasm")]
mod loader {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(inline_js = "
        export function show_loader() {
            document.querySelector('.loader').hidden = false;
        }
        export function hide_loader() {
            document.querySelector('.loader').hidden = true;
        }")]
    extern "C" {
        pub(crate) fn show_loader();

        pub(crate) fn hide_loader();
    }
}
