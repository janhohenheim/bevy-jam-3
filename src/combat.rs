use bevy::prelude::*;
pub use components::*;

pub mod components;
mod decision;
mod execution;

pub fn combat_plugin(app: &mut App) {
    app.register_type::<Combatant>()
        .register_type::<Tendency>()
        .register_type::<MoveIndex>()
        .register_type::<Choreography>()
        .register_type::<Move>()
        .register_type::<MoveDuration>()
        .register_type::<components::Condition>()
        .register_type::<CombatantState>()
        .register_type::<ConditionTracker>()
        .add_event::<MoveEvent>()
        .add_systems(
            (
                decision::decide_choreography,
                execution::execute_move,
                execution::execute_choreography,
            )
                .chain()
                .in_base_set(CoreSet::PostUpdate),
        );
}
