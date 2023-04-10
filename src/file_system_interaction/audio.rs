use crate::file_system_interaction::asset_loading::AudioAssets;
use crate::world_interaction::room::{EnterRoomEvent, RoomClearEvent};
use crate::GameState;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_kira_audio::prelude::{Audio, *};
use bevy_mod_sysfail::macros::*;
use std::time::Duration;

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
    //pub(crate) outro: Handle<AudioInstance>,
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

    /*let outro = audio
            .play(audio_assets.outro.clone())
            .with_volume(0.8)
            .handle();
    */
    commands.insert_resource(AudioHandles {
        walking,
        intro_and_loop,
        intro_and_loop_fast,
        fast_loop_only,
        //outro,
    });
}

#[sysfail(log(level = "error"))]
fn handle_audio_events(
    time: Res<Time>,
    mut room_entered_event: EventReader<EnterRoomEvent>,
    mut room_cleared_event: EventReader<RoomClearEvent>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    audio_handles: Res<AudioHandles>,
    mut play_next_outro: Local<bool>,
    mut intro_start_time: Local<f32>,
) -> Result<()> {
    for _ in room_entered_event.iter() {
        /* let outro = audio_instances
            .get_mut(&audio_handles.outro)
            .context("Failed to get audio instance from handle")?;
        outro.stop(default());*/
        let intro = audio_instances
            .get_mut(&audio_handles.intro_and_loop)
            .context("Failed to get audio instance from handle")?;
        intro.resume(default());
        *intro_start_time = time.elapsed_seconds();
    }
    for _ in room_cleared_event.iter() {
        *play_next_outro = true;
    }
    if *play_next_outro {
        let mut next_loop_timestamp = *intro_start_time + 53.38;
        while next_loop_timestamp < time.elapsed_seconds() {
            next_loop_timestamp += 64.05 - 53.38;
        }
        let delta = next_loop_timestamp - time.elapsed_seconds();
        if delta < 0.5 {
            let intro = audio_instances
                .get_mut(&audio_handles.intro_and_loop)
                .context("Failed to get audio instance from handle")?;
            let tween = AudioTween::linear(Duration::from_secs_f32(delta));
            //intro.pause(tween.clone());
            /*let outro = audio_instances
                .get_mut(&audio_handles.outro)
                .context("Failed to get audio instance from handle")?;
            outro.resume(tween);*/
            *play_next_outro = false;
        }
    }
    Ok(())
}
