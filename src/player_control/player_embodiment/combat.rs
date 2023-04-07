use crate::level_instantiation::spawning::AnimationEntityLink;
use crate::player_control::player_embodiment::PlayerAction;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_mod_sysfail::sysfail;
use leafwing_input_manager::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Bundle)]
pub struct PlayerCombatBundle {
    pub player_combat: PlayerCombatState,
    pub player_combat_animations: PlayerCombatAnimations,
}

#[derive(Debug, Clone, Copy, Component, Reflect, FromReflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct PlayerCombatState {
    pub kind: PlayerCombatKind,
    pub buffer: Option<PlayerCombatKind>,
    pub cancellation: Cancellation,
    pub time_in_state: f32,
}

impl PlayerCombatState {
    pub fn use_next_kind(&mut self, kind: PlayerCombatKind) {
        *self = Self { kind, ..default() };
    }
}

#[derive(
    Debug, Clone, Copy, Reflect, FromReflect, Serialize, Deserialize, Default, Eq, PartialEq,
)]
#[reflect(Serialize, Deserialize)]
pub enum Cancellation {
    Cancelable,
    InBufferPeriod,
    #[default]
    NotCancelable,
}

#[derive(Debug, Clone, Component, Reflect, FromReflect, Default)]
#[reflect(Component)]
pub struct PlayerCombatAnimations {
    pub idle: Handle<AnimationClip>,
    pub attacks: [Handle<AnimationClip>; 3],
    pub block: Handle<AnimationClip>,
    pub hurt: Handle<AnimationClip>,
}

#[derive(
    Debug, Clone, Copy, Reflect, FromReflect, Serialize, Deserialize, Default, Eq, PartialEq,
)]
#[reflect(Serialize, Deserialize)]
pub enum PlayerCombatKind {
    #[default]
    Idle,
    Attack(u16),
    Block,
    Hurt,
}

impl PlayerCombatKind {
    pub fn get_animation<'a>(
        &self,
        animations: &'a PlayerCombatAnimations,
    ) -> &'a Handle<AnimationClip> {
        match self {
            PlayerCombatKind::Idle => &animations.idle,
            PlayerCombatKind::Attack(attack) => &animations.attacks[*attack as usize],
            PlayerCombatKind::Block => &animations.block,
            PlayerCombatKind::Hurt => &animations.hurt,
        }
    }
}

pub fn attack(
    mut players: Query<(
        &ActionState<PlayerAction>,
        &mut PlayerCombatState,
        &PlayerCombatAnimations,
    )>,
) {
    for (actions, mut combat_state, animations) in players.iter_mut() {
        if actions.just_released(PlayerAction::Attack) {
            let desired_state = if let PlayerCombatKind::Attack(attack) = combat_state.kind {
                let next_attack = if attack + 1 < animations.attacks.len() as u16 {
                    attack + 1
                } else {
                    0
                };
                PlayerCombatKind::Attack(next_attack)
            } else {
                PlayerCombatKind::Attack(0)
            };
            match combat_state.cancellation {
                Cancellation::Cancelable => combat_state.use_next_kind(desired_state),
                Cancellation::InBufferPeriod => {
                    combat_state.buffer = Some(desired_state);
                }
                Cancellation::NotCancelable => {}
            }
        }
    }
}

pub fn update_states(
    time: Res<Time>,
    mut players: Query<(&mut PlayerCombatState, &PlayerCombatAnimations)>,
    animations: Res<Assets<AnimationClip>>,
) {
    for (mut combat_state, animation_handles) in players.iter_mut() {
        combat_state.time_in_state += time.delta_seconds();
        let animation_handle = combat_state.kind.get_animation(animation_handles);
        if let Some(animation) = animations.get(animation_handle) {
            if combat_state.time_in_state > animation.duration() {
                combat_state.time_in_state = 0.0;
                combat_state.kind = PlayerCombatKind::Idle;
            }
        }
    }
}

#[sysfail(log(level = "error"))]
pub fn set_animations(
    mut players: Query<(
        &PlayerCombatState,
        &PlayerCombatAnimations,
        &AnimationEntityLink,
    )>,
    mut animation_players: Query<&mut AnimationPlayer>,
) -> Result<()> {
    for (combat_state, animations, animation_entity_link) in players.iter_mut() {
        let mut animation_player = animation_players.get_mut(animation_entity_link.0).context(
            "Animation entity link points to an entity that does not have an animation player",
        )?;
        let animation = combat_state.kind.get_animation(animations).clone();
        animation_player.play_with_transition(animation, Duration::from_secs_f32(0.1));
    }
    Ok(())
}
