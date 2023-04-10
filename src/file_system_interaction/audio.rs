use crate::file_system_interaction::asset_loading::AudioAssets;
use crate::world_interaction::room::EnterRoomEvent;
use crate::GameState;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_kira_audio::prelude::{Audio, *};
use bevy_mod_sysfail::macros::*;

/// Handles initialization of all sounds.
pub(crate) fn internal_audio_plugin(app: &mut App) {
    app.add_plugin(AudioPlugin)
        .add_system(init_audio.in_schedule(OnExit(GameState::Loading)))
        .add_system(handle_audio_events.in_set(OnUpdate(GameState::Playing)));
}

#[derive(Debug, Clone, Resource)]
pub(crate) struct AudioHandles {
    pub(crate) walking: Handle<AudioInstance>,
    pub(crate) intro_and_loop: Handle<AudioInstance>,
    pub(crate) intro_and_loop_fast: Handle<AudioInstance>,
    pub(crate) fast_loop_only: Handle<AudioInstance>,
}

fn init_audio(mut commands: Commands, audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.pause();
    let walking = audio
        .play(audio_assets.walking.clone())
        .looped()
        .with_volume(0.8)
        .handle();

    let intro_and_loop = audio
        .play(audio_assets.intro_and_loop.clone())
        .loop_from(53.38)
        .with_volume(0.8)
        .handle();
    // Whole thing is 64.05

    let intro_and_loop_fast = audio
        .play(audio_assets.intro_and_loop_fast.clone())
        .loop_from(63.10)
        .with_volume(0.8)
        .handle();
    // intro_and_loop at end -> intro_and_loop_fast at 49.37

    let fast_loop_only = audio
        .play(audio_assets.fast_loop_only.clone())
        .loop_from(13.78)
        .with_volume(0.8)
        .handle();
    // intro_and_loop at end -> fast_loop_only at start

    commands.insert_resource(AudioHandles {
        walking,
        intro_and_loop,
        intro_and_loop_fast,
        fast_loop_only,
    });
}

#[sysfail(log(level = "error"))]
fn handle_audio_events(
    mut room_entered_event: EventReader<EnterRoomEvent>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    audio_handles: Res<AudioHandles>,
) -> Result<()> {
    for _ in room_entered_event.iter() {
        let intro = audio_instances
            .get_mut(&audio_handles.intro_and_loop)
            .context("Failed to get audio instance from handle")?;
        intro.resume(default());
        intro.seek_to(0.0).context("Failed to seek start")?;
    }
    Ok(())
}
