use crate::level_instantiation::spawning::animation_link::link_animations;
use crate::GameState;
use bevy::prelude::*;
pub use components::*;

pub mod components;
#[cfg(feature = "dev")]
pub mod debug;
mod decision;
mod execution;
mod update_states;

pub fn combat_plugin(app: &mut App) {
    app.register_type::<Combatant>()
        .register_type::<Tendency>()
        .register_type::<CurrentMove>()
        .register_type::<Choreography>()
        .register_type::<Move>()
        .register_type::<MoveDuration>()
        .register_type::<components::Condition>()
        .register_type::<CombatantState>()
        .register_type::<ConditionTracker>()
        .add_event::<MoveEvent>()
        .add_systems(
            (
                update_states::update_condition_tracker,
                decision::decide_choreography,
                execution::execute_move,
                execution::execute_choreography,
                #[cfg(feature = "dev")]
                debug::display_combatants,
            )
                .chain()
                .after(link_animations)
                .in_set(OnUpdate(GameState::Playing)),
        );
}
