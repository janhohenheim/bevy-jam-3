use crate::combat::Enemy;
use crate::file_system_interaction::audio::AudioHandles;
use crate::level_instantiation::spawning::GameObject;
use crate::player_control::player_embodiment::Player;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::AudioInstance;
use spew::prelude::SpawnEvent;

pub(crate) fn exit_plugin(app: &mut App) {
    app.add_event::<EnterRoomEvent>()
        .add_event::<RoomClearEvent>()
        .add_event::<LeaveRoomEvent>()
        .register_type::<CurrentRoom>()
        .init_resource::<CurrentRoom>()
        .add_systems(
            (enter_first_room, update_room, leave_room).in_set(OnUpdate(GameState::Playing)),
        );
}

#[derive(Debug, Clone, Component)]
pub(crate) struct Exit;

#[derive(Debug, Clone, Component)]
pub(crate) struct Room;

#[derive(Debug, Clone, Resource, Reflect, FromReflect, Default)]
#[reflect(Resource)]
pub struct CurrentRoom {
    pub(crate) cleared: bool,
    pub(crate) number: usize,
}

impl CurrentRoom {
    pub(crate) fn enter_next(&mut self) {
        self.number += 1;
        self.cleared = false;
    }
}

#[derive(Debug, Clone)]
pub(crate) struct EnterRoomEvent;

#[derive(Debug, Clone)]
pub(crate) struct RoomClearEvent;

#[derive(Debug, Clone)]
pub(crate) struct LeaveRoomEvent;

fn enter_first_room(
    mut start_room_events: EventWriter<EnterRoomEvent>,
    mut loaded: Local<bool>,
    audio_instances: Res<Assets<AudioInstance>>,
    audio_handles: Res<AudioHandles>,
) {
    if *loaded {
        return;
    }
    if audio_instances.get(&audio_handles.intro_and_loop).is_some() {
        start_room_events.send(EnterRoomEvent);
        *loaded = true;
    }
}

fn update_room(
    enemies: Query<(), With<Enemy>>,
    players: Query<(), With<Player>>,
    mut room_clear_events: EventWriter<RoomClearEvent>,
    mut current_room: ResMut<CurrentRoom>,
) {
    if !players.is_empty() && enemies.is_empty() {
        room_clear_events.send(RoomClearEvent);
        current_room.cleared = true;
    }
}

fn leave_room(
    mut commands: Commands,
    mut leave_room_events: EventReader<LeaveRoomEvent>,
    mut current_room: ResMut<CurrentRoom>,
    mut spawn_events: EventWriter<SpawnEvent<GameObject, Transform>>,
    rooms: Query<Entity, With<Room>>,
) {
    for _ in leave_room_events.iter() {
        current_room.enter_next();
        spawn_events.send(SpawnEvent::new(GameObject::IntroRoom));
        for room in rooms.iter() {
            commands.entity(room).despawn_recursive();
        }
    }
}
