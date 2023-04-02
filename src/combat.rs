use crate::level_instantiation::spawning::AnimationEntityLink;
use crate::GameState;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_mod_sysfail::sysfail;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub fn combat_plugin(app: &mut App) {
    app.register_type::<CombatantState>()
        .register_type::<Choreography>()
        .register_type::<Move>()
        .add_event::<MoveEvent>()
        .add_systems((execute_move,).in_set(OnUpdate(GameState::Playing)));
}

#[sysfail(log(level = "error"))]
fn execute_move(
    mut move_events: EventReader<MoveEvent>,
    mut animation_player: Query<&mut AnimationPlayer>,
    mut attackers: Query<(&AnimationEntityLink, &mut CombatantState)>,
) -> Result<()> {
    for event in move_events.iter() {
        let move_ = &event.move_;
        let (animation_entity_link, mut combatant_state) = attackers.get_mut(event.source)?;
        let mut animation_player = animation_player
            .get_mut(**animation_entity_link)
            .context("animation_entity_link held entity without animation player")?;

        animation_player
            .play_with_transition(move_.animation.clone(), Duration::from_secs_f32(0.2));
        *combatant_state = move_.state;
    }
    Ok(())
}

#[derive(Debug, Component, Clone, PartialEq, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub struct Choreography(pub Vec<Move>);

#[derive(Debug, Component, Clone, PartialEq, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub struct Move {
    duration: f32,
    pub animation: Handle<AnimationClip>,
    pub state: CombatantState,
}

#[derive(Debug, Component, Clone, PartialEq)]
pub struct MoveEvent {
    pub source: Entity,
    pub move_: Move,
}

#[derive(
    Debug, Component, Clone, Copy, PartialEq, Default, Reflect, FromReflect, Serialize, Deserialize,
)]
#[reflect(Component, Serialize, Deserialize)]
pub enum CombatantState {
    Deathblow,
    Vulnerable,
    #[default]
    OnGuard,
    HyperArmor,
}
