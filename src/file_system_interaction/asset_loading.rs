use crate::file_system_interaction::config::GameConfig;
use crate::file_system_interaction::level_serialization::SerializedLevel;
use crate::world_interaction::dialog::Dialog;
use crate::GameState;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_common_assets::toml::TomlAssetPlugin;
use bevy_egui::egui::ProgressBar;
use bevy_egui::{egui, EguiContexts};
use bevy_kira_audio::AudioSource;
use bevy_mod_sysfail::macros::*;
use iyes_progress::{ProgressCounter, ProgressPlugin};

pub(crate) fn loading_plugin(app: &mut App) {
    app.add_plugin(RonAssetPlugin::<SerializedLevel>::new(&["lvl.ron"]))
        .add_plugin(RonAssetPlugin::<Dialog>::new(&["dlg.ron"]))
        .add_plugin(TomlAssetPlugin::<GameConfig>::new(&["game.toml"]))
        .add_plugin(ProgressPlugin::new(GameState::Loading).continue_to(GameState::Menu))
        .add_loading_state(LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu))
        .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, SceneAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, DummyAnimationAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, FpsDummyAnimationAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, LevelAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, DialogAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, ConfigAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, RoomAssets>(GameState::Loading)
        .add_system(show_progress.in_set(OnUpdate(GameState::Loading)))
        .add_system(update_config);
}

// the following asset collections will be loaded during the State `GameState::InitialLoading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct AudioAssets {
    #[asset(path = "audio/walking.ogg")]
    pub(crate) walking: Handle<AudioSource>,
    #[asset(path = "audio/intro_and_loop.ogg")]
    pub(crate) intro_and_loop: Handle<AudioSource>,
    #[asset(path = "audio/intro_and_loop_fast.ogg")]
    pub(crate) intro_and_loop_fast: Handle<AudioSource>,
    #[asset(path = "audio/fast_loop_only.ogg")]
    pub(crate) fast_loop_only: Handle<AudioSource>,
    //#[asset(path = "audio/outro.ogg")]
    //pub(crate) outro: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct SceneAssets {
    #[asset(path = "scenes/fps_dummy.glb#Scene0")]
    pub(crate) fps_dummy: Handle<Scene>,
    #[asset(path = "scenes/dummy.glb#Scene0")]
    pub(crate) dummy: Handle<Scene>,
    #[asset(path = "scenes/kunai.glb#Scene0")]
    pub(crate) kunai: Handle<Scene>,
}

#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct RoomAssets {
    #[asset(path = "scenes/intro_room.glb#Scene0")]
    pub(crate) intro: Handle<Scene>,
    #[asset(path = "scenes/room_one.glb#Scene0")]
    pub(crate) one: Handle<Scene>,
    #[asset(path = "scenes/room_two.glb#Scene0")]
    pub(crate) two: Handle<Scene>,
    #[asset(path = "scenes/room_three.glb#Scene0")]
    pub(crate) three: Handle<Scene>,
}

#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct DummyAnimationAssets {
    #[asset(path = "scenes/dummy.glb#Animation0")]
    pub(crate) hurt: Handle<AnimationClip>,
    #[asset(path = "scenes/dummy.glb#Animation1")]
    pub(crate) block: Handle<AnimationClip>,
    #[asset(path = "scenes/dummy.glb#Animation2")]
    pub(crate) aerial_toss: Handle<AnimationClip>,
    #[asset(path = "scenes/dummy.glb#Animation3")]
    pub(crate) aerial: Handle<AnimationClip>,
    #[asset(path = "scenes/dummy.glb#Animation4")]
    pub(crate) attack: Handle<AnimationClip>,
    #[asset(path = "scenes/dummy.glb#Animation5")]
    pub(crate) walk: Handle<AnimationClip>,
    #[asset(path = "scenes/dummy.glb#Animation6")]
    pub(crate) idle: Handle<AnimationClip>,
}

#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct FpsDummyAnimationAssets {
    #[asset(path = "scenes/fps_dummy.glb#Animation0")]
    pub(crate) idle: Handle<AnimationClip>,
    #[asset(path = "scenes/fps_dummy.glb#Animation1")]
    pub(crate) attack_one: Handle<AnimationClip>,
    #[asset(path = "scenes/fps_dummy.glb#Animation2")]
    pub(crate) attack_two: Handle<AnimationClip>,
    #[asset(path = "scenes/fps_dummy.glb#Animation3")]
    pub(crate) attack_three: Handle<AnimationClip>,
    #[asset(path = "scenes/fps_dummy.glb#Animation4")]
    pub(crate) block: Handle<AnimationClip>,
    #[asset(path = "scenes/fps_dummy.glb#Animation5")]
    pub(crate) hurt: Handle<AnimationClip>,
    #[asset(path = "scenes/fps_dummy.glb#Animation6")]
    pub(crate) blocked: Handle<AnimationClip>,
}

#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct LevelAssets {
    #[cfg_attr(feature = "native", asset(path = "levels", collection(typed, mapped)))]
    #[cfg_attr(
        feature = "wasm",
        asset(paths("levels/intro_room.lvl.ron"), collection(typed, mapped))
    )]
    pub(crate) levels: HashMap<String, Handle<SerializedLevel>>,
}

#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct DialogAssets {
    #[cfg_attr(feature = "native", asset(path = "dialogs", collection(typed, mapped)))]
    #[cfg_attr(
        feature = "wasm",
        asset(paths("dialogs/follower.dlg.ron"), collection(typed, mapped))
    )]
    pub(crate) dialogs: HashMap<String, Handle<Dialog>>,
}

#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct TextureAssets {
    #[asset(path = "textures/stone_alley_2.jpg")]
    pub(crate) glowy_interior: Handle<Image>,
    #[asset(path = "textures/sky.jpg")]
    pub(crate) sky: Handle<Image>,
    #[asset(path = "textures/bar_border.png")]
    pub(crate) bar_border: Handle<Image>,
    #[asset(path = "textures/health_bar_fill.png")]
    pub(crate) health_bar_fill: Handle<Image>,
    #[asset(path = "textures/posture_bar_fill.png")]
    pub(crate) posture_bar_fill: Handle<Image>,
    #[asset(path = "textures/posture_bar_top.png")]
    pub(crate) posture_bar_top: Handle<Image>,
}

#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct ConfigAssets {
    #[asset(path = "config/config.game.toml")]
    pub(crate) _game: Handle<GameConfig>,
}

fn show_progress(
    progress: Option<Res<ProgressCounter>>,
    mut egui_contexts: EguiContexts,
    mut last_done: Local<u32>,
    audio_assets: Option<Res<AudioAssets>>,
    scene_assets: Option<Res<SceneAssets>>,
    level_assets: Option<Res<LevelAssets>>,
    dialog_assets: Option<Res<DialogAssets>>,
    texture_assets: Option<Res<TextureAssets>>,
    config_assets: Option<Res<ConfigAssets>>,
    dummy_animation_assets: Option<Res<DummyAnimationAssets>>,
    fps_dummy_animation_assets: Option<Res<FpsDummyAnimationAssets>>,
    room_assets: Option<Res<RoomAssets>>,
) {
    if let Some(progress) = progress.map(|counter| counter.progress()) {
        if progress.done > *last_done {
            *last_done = progress.done;
        }

        egui::CentralPanel::default().show(egui_contexts.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                ui.heading("Loading");
                ui.label("Loading assets...");
                ui.add(
                    ProgressBar::new(progress.done as f32 / progress.total as f32).animate(true),
                );
                ui.add_space(100.0);
                ui.add_enabled_ui(false, |ui| {
                    ui.checkbox(&mut audio_assets.is_some(), "Audio");
                    ui.checkbox(&mut scene_assets.is_some(), "Scenes");
                    ui.checkbox(&mut level_assets.is_some(), "Levels");
                    ui.checkbox(&mut dialog_assets.is_some(), "Dialogs");
                    ui.checkbox(&mut texture_assets.is_some(), "Textures");
                    ui.checkbox(&mut config_assets.is_some(), "Config");
                    ui.checkbox(&mut room_assets.is_some(), "Rooms");
                    ui.checkbox(&mut dummy_animation_assets.is_some(), "Dummy Animations");
                    ui.checkbox(
                        &mut fps_dummy_animation_assets.is_some(),
                        "FPS Dummy Animations",
                    );
                });
            });
        });
    }
}

#[sysfail(log(level = "error"))]
fn update_config(
    mut commands: Commands,
    config: Res<Assets<GameConfig>>,
    mut config_asset_events: EventReader<AssetEvent<GameConfig>>,
) -> Result<()> {
    #[cfg(feature = "tracing")]
    let _span = info_span!("update_config").entered();
    for event in config_asset_events.iter() {
        match event {
            AssetEvent::Created { handle } | AssetEvent::Modified { handle } => {
                // Guaranteed by Bevy to not fail
                let config = config
                    .get(handle)
                    .context("Failed to get config even though it was just created")?;
                commands.insert_resource(config.clone());
            }
            AssetEvent::Removed { .. } => {}
        }
    }
    Ok(())
}
