use crate::ai::generic::projectile::spawn_actual_simple_projectile;
use crate::combat::collision::{EnemyHitEvent, HitCache, HitboxHits, PlayerHitEvent};
use crate::level_instantiation::spawning::animation_link::link_animations;
use crate::movement::general_movement::reset_forces_and_impulses;
use crate::GameState;
use bevy::prelude::*;
pub use components::*;
use spew::prelude::*;

pub(crate) mod collision;
pub mod components;
#[cfg(feature = "dev")]
pub mod debug;
mod decision;
mod execution;
mod linking;
mod update_states;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct CombatSystemSet;

pub fn combat_plugin(app: &mut App) {
    app.register_type::<CurrentMove>()
        .register_type::<CombatantState>()
        .register_type::<ConditionTracker>()
        .register_type::<MoveMetadata>()
        .register_type::<AttackHitbox>()
        .register_type::<Attack>()
        .register_type::<Projectile>()
        .register_type::<ProjectileSpawnInput>()
        .register_type::<PlayerHitEvent>()
        .register_type::<EnemyHitEvent>()
        .register_type::<Constitution>()
        .register_type::<HitCache>()
        .register_type::<HitboxHits>()
        .add_event::<PlayerHitEvent>()
        .add_event::<EnemyHitEvent>()
        .add_event::<InitMoveEvent>()
        .add_event::<ExecuteMoveEvent>()
        .add_plugin(SpewPlugin::<ProjectileKind, (Entity, ProjectileSpawnInput)>::default())
        .add_spawners(((ProjectileKind::Simple, spawn_actual_simple_projectile),))
        .init_resource::<HitCache>()
        .add_systems(
            (
                linking::link_hitbox,
                collision::clear_cache,
                collision::detect_hits,
                collision::handle_enemy_being_hit,
                update_states::update_condition_tracker,
                decision::decide_choreography,
                execution::execute_choreography,
                execution::init_move,
                execution::execute_move,
                linking::sync_projectile_attack_hitbox,
                #[cfg(feature = "dev")]
                debug::display_combatants,
            )
                .chain()
                .after(link_animations)
                .after(reset_forces_and_impulses)
                .in_set(OnUpdate(GameState::Playing))
                .in_set(CombatSystemSet),
        );
}
