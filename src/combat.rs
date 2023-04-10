use crate::ai::generic::projectile::spawn_actual_simple_projectile;
use crate::combat::collision::{
    BlockedByEnemyEvent, DeflectedByEnemyEvent, EnemyHitEvent, EnemyHurtEvent, HitCache,
    HitboxHits, PlayerHitEvent,
};
use crate::level_instantiation::spawning::animation_link::link_animations;
use crate::movement::general_movement::reset_forces_and_impulses;
use crate::util::criteria::never;
use crate::GameState;
use bevy::prelude::*;
pub(crate) use components::*;
use seldom_fn_plugin::FnPluginExt;
use spew::prelude::*;

pub(crate) mod collision;
pub(crate) mod components;
mod constitution;
pub(crate) mod debug;
mod decision;
mod execution;
mod linking;
mod ui;
mod update_states;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub(crate) struct CombatSystemSet;

pub(crate) fn combat_plugin(app: &mut App) {
    app.register_type::<CurrentMove>()
        .register_type::<EnemyCombatState>()
        .register_type::<ConditionTracker>()
        .register_type::<CurrentMoveMetadata>()
        .register_type::<AttackHitbox>()
        .register_type::<Attack>()
        .register_type::<Projectile>()
        .register_type::<ProjectileSpawnInput>()
        .register_type::<PlayerHitEvent>()
        .register_type::<EnemyHitEvent>()
        .register_type::<Constitution>()
        .register_type::<HitCache>()
        .register_type::<HitboxHits>()
        .register_type::<HitboxParentModel>()
        .add_event::<PlayerHitEvent>()
        .add_event::<EnemyHitEvent>()
        .add_event::<ReadMoveMetadataEvent>()
        .add_event::<ExecuteMoveFunctionsEvent>()
        .add_event::<EnemyHurtEvent>()
        .add_event::<BlockedByEnemyEvent>()
        .add_event::<DeflectedByEnemyEvent>()
        .add_plugin(SpewPlugin::<ProjectileKind, (Entity, ProjectileSpawnInput)>::default())
        .add_spawners(((ProjectileKind::Simple, spawn_actual_simple_projectile),))
        .init_resource::<HitCache>()
        .fn_plugin(ui::enemy_combat_ui_plugin)
        .add_systems(
            (
                linking::link_hitbox,
                collision::clear_cache,
                collision::detect_hits,
                collision::handle_enemy_being_hit,
                collision::handle_hurt_events,
                collision::handle_block_events,
                collision::handle_deflect_events,
                update_states::update_condition_tracker,
                decision::decide_choreography,
                execution::execute_choreography,
                execution::read_move_metadata,
                execution::execute_move_functions,
                linking::sync_projectile_attack_hitbox,
            )
                .chain()
                .after(link_animations)
                .after(reset_forces_and_impulses)
                .in_set(OnUpdate(GameState::Playing))
                .in_set(CombatSystemSet),
        )
        .add_systems(
            (constitution::update_posture, constitution::handle_death)
                .chain()
                .after(link_animations)
                .after(reset_forces_and_impulses)
                .in_set(OnUpdate(GameState::Playing))
                .in_set(CombatSystemSet),
        )
        .add_system(
            debug::display_combatants
                .run_if(never)
                .in_set(OnUpdate(GameState::Playing)),
        );
}
