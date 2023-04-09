use crate::GameState;
use bevy::prelude::*;

pub(crate) mod generic;

pub(crate) fn ai_plugin(app: &mut App) {
    app.add_systems(
        (
            generic::projectile::behavior::fly_toward_player,
            generic::projectile::behavior::handle_projectile_lifetimes,
        )
            .chain()
            .in_set(OnUpdate(GameState::Playing)),
    );
}
