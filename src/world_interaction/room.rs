use crate::combat::Enemy;
use crate::file_system_interaction::audio::AudioHandles;
use crate::level_instantiation::spawning::GameObject;
use crate::player_control::actions::ActionsFrozen;
use crate::player_control::player_embodiment::Player;
use crate::world_interaction::side_effects::potions::{generate_potions, Potion, POTION_COUNT};
use crate::world_interaction::side_effects::SideEffects;
use crate::GameState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_kira_audio::AudioInstance;
use rand::Rng;
use spew::prelude::SpawnEvent;

pub(crate) fn exit_plugin(app: &mut App) {
    app.add_event::<EnterRoomEvent>()
        .add_event::<RoomClearEvent>()
        .add_event::<LeaveRoomEvent>()
        .add_event::<SelectPotionEvent>()
        .register_type::<CurrentRoom>()
        .register_type::<SelectPotionUi>()
        .init_resource::<CurrentRoom>()
        .init_resource::<SelectPotionUi>()
        .add_systems(
            (
                enter_first_room,
                update_room,
                leave_room,
                activate_select_potion_ui,
                select_potion,
            )
                .chain()
                .in_set(OnUpdate(GameState::Playing)),
        );
}

#[derive(Debug, Clone, Component)]
pub(crate) struct Exit;

#[derive(Debug, Clone, Component)]
pub(crate) struct Room;

#[derive(Debug, Clone, Resource, Reflect, FromReflect, Default)]
#[reflect(Resource)]
pub(crate) struct CurrentRoom {
    pub(crate) cleared: bool,
    pub(crate) number: usize,
}

impl CurrentRoom {
    pub(crate) fn enter_next(&mut self) {
        self.number += 1;
        self.cleared = false;
    }
}

#[derive(Debug, Clone, Resource, Reflect, FromReflect, Default)]
#[reflect(Resource)]
struct SelectPotionUi {
    potions: Option<[Potion; POTION_COUNT]>,
}

#[derive(Debug, Clone)]
pub(crate) struct EnterRoomEvent;

#[derive(Debug, Clone)]
pub(crate) struct RoomClearEvent;

#[derive(Debug, Clone)]
pub(crate) struct LeaveRoomEvent;

#[derive(Debug, Clone)]
pub(crate) struct SelectPotionEvent;

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

fn activate_select_potion_ui(
    mut events: EventReader<SelectPotionEvent>,
    mut select_potion_ui: ResMut<SelectPotionUi>,
) {
    for _ in events.iter() {
        select_potion_ui.potions = Some(generate_potions());
    }
}

fn select_potion(
    mut egui_contexts: EguiContexts,
    mut leave_room_events: EventWriter<LeaveRoomEvent>,
    mut select_potion_ui: ResMut<SelectPotionUi>,
    mut side_effects: ResMut<SideEffects>,
) {
    let Some(potions) = select_potion_ui.potions.clone() else {
        return;
    };
    egui::Window::new("Select a Potion")
        .resizable(false)
        .collapsible(false)
        .default_pos([400., 400.])
        .show(egui_contexts.ctx_mut(), |ui| {
            ui.label("The pirates left some potions behind. Which one do you want to take?");
            ui.separator();
            egui::Grid::new("potion_grid")
                .min_col_width(50.)
                .spacing([30., 3.])
                .show(ui, |ui| {
                    for potion in potions.iter() {
                        ui.heading(&potion.name);
                    }
                    ui.end_row();
                    for potion in potions.iter() {
                        ui.label(&potion.positive_side_effect.format_positive());
                    }
                    ui.end_row();
                    for _potion in potions.iter() {
                        ui.label("BUT");
                    }
                    ui.end_row();
                    for potion in potions.iter() {
                        ui.label(&potion.negative_side_effect.format_negative());
                    }
                    ui.end_row();
                    for potion in potions.iter() {
                        ui.horizontal_centered(|ui| {
                            if ui.button("Drink!").clicked() {
                                side_effects.add_positive(potion.positive_side_effect);
                                side_effects.add_negative(potion.negative_side_effect);
                                select_potion_ui.potions = None;
                                leave_room_events.send(LeaveRoomEvent);
                            }
                        });
                    }
                });
        });
}

fn leave_room(
    mut commands: Commands,
    mut leave_room_events: EventReader<LeaveRoomEvent>,
    mut current_room: ResMut<CurrentRoom>,
    mut spawn_events: EventWriter<SpawnEvent<GameObject, Transform>>,
    rooms: Query<Entity, With<Room>>,
    mut actions_frozen: ResMut<ActionsFrozen>,
    mut enter_room_events: EventWriter<EnterRoomEvent>,
) {
    for _ in leave_room_events.iter() {
        actions_frozen.unfreeze();
        current_room.enter_next();
        spawn_events.send(SpawnEvent::new(choose_room()));
        enter_room_events.send(EnterRoomEvent);
        for room in rooms.iter() {
            commands.entity(room).despawn_recursive();
        }
    }
}

fn choose_room() -> GameObject {
    let mut rng = rand::thread_rng();
    let room = rng.gen_range(0..=3);
    info!("Choosing room {}", room);
    match room {
        0 => GameObject::IntroRoom,
        1 => GameObject::RoomOne,
        2 => GameObject::RoomTwo,
        3 => GameObject::RoomThree,
        _ => GameObject::IntroRoom,
    }
}
