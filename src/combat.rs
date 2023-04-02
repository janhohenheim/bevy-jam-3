use bevy::prelude::*;

mod components;
pub use components::*;

mod decision;
mod execution;

pub fn combat_plugin(app: &mut App) {
    app.register_type::<CombatantState>()
        .register_type::<Choreography>()
        .register_type::<Move>()
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
