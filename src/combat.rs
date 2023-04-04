use crate::level_instantiation::spawning::animation_link::link_animations;
use crate::movement::general_movement::GeneralMovementSystemSet;
use crate::GameState;
use bevy::prelude::*;
pub use components::*;
use spew::prelude::*;

pub mod components;
#[cfg(feature = "dev")]
pub mod debug;
mod decision;
mod execution;
mod linking;
mod update_states;

pub fn combat_plugin(app: &mut App) {
    app.register_type::<CurrentMove>()
        .register_type::<CombatantState>()
        .register_type::<ConditionTracker>()
        .add_event::<InitMoveEvent>()
        .add_event::<ExecuteMoveEvent>()
        .add_plugin(SpewPlugin::<ProjectileKind, ProjectileSpawnInput>::default())
        .add_systems(
            (
                linking::link_melee_attack,
                update_states::update_condition_tracker,
                decision::decide_choreography,
                execution::init_move,
                execution::execute_move,
                execution::execute_choreography,
                #[cfg(feature = "dev")]
                debug::display_combatants,
            )
                .chain()
                .after(link_animations)
                .after(GeneralMovementSystemSet)
                .in_set(OnUpdate(GameState::Playing)),
        );
}
