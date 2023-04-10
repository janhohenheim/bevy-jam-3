use crate::movement::general_movement::ManualRotation;
use bevy::prelude::*;
use bevy::utils::HashMap;
pub(crate) use condition::*;
pub(crate) use move_::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

mod condition;
mod move_;

#[derive(Debug, Clone, Bundle, Default)]
pub(crate) struct CombatBundle {
    pub(crate) enemy: Enemy,
    pub(crate) combat_state: EnemyCombatState,
    pub(crate) condition_tracker: ConditionTracker,
    pub(crate) current_move_metadata: CurrentMoveMetadata,
    pub(crate) manual_rotation: ManualRotation,
    pub(crate) constitution: Constitution,
}

#[derive(Debug, Component, Clone, Default)]
pub(crate) struct Enemy {
    pub(crate) choreographies: Vec<Choreography>,
    pub(crate) last_choreography: Option<usize>,
    pub(crate) current: Option<CurrentMove>,
    pub(crate) tendencies: Vec<Tendency>,
    /// Used to implement e.g. circling around player after a strong boss attack.
    /// Currently does not factor in any conditions.
    pub(crate) chained_choreographies: HashMap<usize, usize>,
    pub(crate) time_since_last_move: f32,
    pub(crate) time_since_last_animation: f32,
    pub(crate) time_since_hurt_or_block: f32,
    pub(crate) forced_choreography: Option<usize>,
    pub(crate) special_choreographies: SpecialChoreographies,
    pub(crate) is_dead: bool,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct SpecialChoreographies {
    pub(crate) hurt: usize,
    pub(crate) block: usize,
    pub(crate) posture_broken: usize,
    pub(crate) death: usize,
}

impl Enemy {
    pub(crate) fn new(
        choreographies: Vec<Choreography>,
        tendencies: Vec<Tendency>,
        chained_choreographies: HashMap<usize, usize>,
        special_choreographies: SpecialChoreographies,
    ) -> Self {
        Self {
            choreographies,
            tendencies,
            chained_choreographies,
            special_choreographies,
            ..default()
        }
    }

    pub(crate) fn update_timers(&mut self, dt: f32) {
        self.time_since_last_move += dt;
        self.time_since_last_animation += dt;
        self.time_since_hurt_or_block += dt;
    }
    pub(crate) fn is_ready_for_next_choreography(&self) -> bool {
        self.current.is_none() || self.forced_choreography.is_some()
    }

    pub(crate) fn current_choreography(&self) -> Option<&Choreography> {
        self.current
            .as_ref()
            .map(|current| &self.choreographies[current.choreography])
    }

    pub(crate) fn current_move(&self) -> Option<&Move> {
        self.current.as_ref().and_then(|current| {
            self.choreographies
                .get(current.choreography)
                .and_then(|choreography| choreography.moves.get(current.move_))
        })
    }

    pub(crate) fn block(&mut self) {
        if self.is_dead {
            return;
        }
        self.forced_choreography = Some(self.special_choreographies.block);
    }

    pub(crate) fn hurt(&mut self) {
        if self.is_dead {
            return;
        }
        self.forced_choreography = Some(self.special_choreographies.hurt);
    }

    pub(crate) fn break_posture(&mut self) {
        if self.is_dead {
            return;
        }
        self.forced_choreography = Some(self.special_choreographies.posture_broken);
    }

    pub(crate) fn die(&mut self) {
        self.forced_choreography = Some(self.special_choreographies.death);
        self.is_dead = true;
    }
}

#[derive(Debug, Clone, Copy, Component, Reflect, FromReflect)]
#[reflect(Component)]
pub(crate) struct Constitution {
    health: f32,
    max_health: f32,
    posture: f32,
    vanilla_posture_recovery: f32,
    vanilla_max_posture: f32,
    vanilla_max_health: f32,
    max_posture: f32,
    base_posture_recovery: f32,
    is_posture_broken: bool,
    is_dead: bool,
}

impl Constitution {
    pub(crate) fn with_max_health(mut self, max_health: f32) -> Self {
        self.max_health = max_health;
        self.health = max_health;
        self.vanilla_max_health = max_health;
        self
    }

    pub(crate) fn with_max_posture(mut self, max_posture: f32) -> Self {
        self.max_posture = max_posture;
        self.vanilla_max_posture = max_posture;
        self
    }

    pub(crate) fn with_base_posture_recovery(mut self, base_posture_recovery: f32) -> Self {
        self.base_posture_recovery = base_posture_recovery;
        self.vanilla_posture_recovery = base_posture_recovery;
        self
    }

    pub(crate) fn apply_health_side_effect(&mut self, side_effect: f32) {
        self.max_health = self.vanilla_max_health * side_effect;
        if side_effect > 1.1 {
            self.recover_health(self.health * side_effect);
        }
    }

    pub(crate) fn apply_posture_side_effect(&mut self, side_effect: f32) {
        self.max_posture = self.vanilla_max_posture * side_effect;
    }

    pub(crate) fn apply_posture_recovery_side_effect(&mut self, side_effect: f32) {
        self.base_posture_recovery = self.vanilla_posture_recovery * side_effect;
    }

    pub(crate) fn health(&self) -> f32 {
        self.health
    }

    pub(crate) fn health_fraction(&self) -> f32 {
        self.health / self.max_health
    }

    pub(crate) fn posture(&self) -> f32 {
        self.posture
    }

    pub(crate) fn posture_fraction(&self) -> f32 {
        self.posture / self.max_posture
    }

    pub(crate) fn take_full_damage(&mut self, attack: &Attack) {
        self.take_posture_damage(attack);
        self.take_health_damage(attack);
    }

    fn take_health_damage(&mut self, attack: &Attack) {
        self.health -= attack.health_damage;
        if self.health < 0.0 {
            self.die();
        }
    }

    pub(crate) fn take_posture_damage(&mut self, attack: &Attack) {
        self.posture += attack.posture_damage;
        if self.posture > self.max_posture {
            self.break_posture();
        }
    }

    pub(crate) fn take_posture_damage_deflecting(&mut self, attack: &Attack) {
        let factor = 0.5;
        self.posture += attack.posture_damage * factor;
        if self.posture > self.max_posture {
            // cannot get posture broken from deflecting
            self.posture = self.max_posture;
        }
    }

    pub(crate) fn recover_health(&mut self, amount: f32) {
        if self.is_dead {
            return;
        }
        self.health += amount;
        if self.health > self.max_health {
            self.health = self.max_health;
        }
    }

    pub(crate) fn recover_posture(&mut self, dt: f32) {
        if self.is_dead {
            return;
        }
        let amount = self.base_posture_recovery * dt * self.get_posture_recovery_factor();
        self.posture -= amount;
        if self.posture < 0.0 {
            self.posture = 0.0;
        }
    }

    /// Source: <https://sekiroshadowsdietwice.wiki.fextralife.com/Posture>
    fn get_posture_recovery_factor(&self) -> f32 {
        let health_fraction = self.health / self.max_health;
        if health_fraction > 0.75 {
            1.
        } else if health_fraction > 0.5 {
            2. / 3.
        } else if health_fraction > 0.25 {
            1. / 3.
        } else {
            0.
        }
    }

    pub(crate) fn die(&mut self) {
        self.is_dead = true;
        self.health = 0.0;
    }

    pub(crate) fn break_posture(&mut self) {
        self.posture = self.max_posture;
        self.is_posture_broken = true;
    }

    pub(crate) fn is_dead(&self) -> bool {
        self.is_dead
    }

    pub(crate) fn mark_broken_posture_as_handled(&mut self) {
        if self.is_dead {
            return;
        }
        self.posture = 0.0;
        self.is_posture_broken = false;
    }

    pub(crate) fn is_posture_broken(&self) -> bool {
        self.is_posture_broken
    }
}

impl Default for Constitution {
    fn default() -> Self {
        Self {
            health: 100.0,
            max_health: 100.0,
            posture: 0.0,
            max_posture: 100.0,
            base_posture_recovery: 20.0,
            is_posture_broken: false,
            is_dead: false,
            vanilla_max_health: 100.0,
            vanilla_max_posture: 100.0,
            vanilla_posture_recovery: 20.0,
        }
    }
}

#[derive(Debug, Component, Clone, Deref, DerefMut)]
pub(crate) struct ParentToHitboxLink(pub(crate) Entity);

#[derive(Debug, Component, Clone, Deref, DerefMut)]
pub(crate) struct HitboxToParentLink(pub(crate) Entity);

#[derive(Debug, Clone, PartialEq, Default)]
pub(crate) struct Tendency {
    pub(crate) choreography: usize,
    pub(crate) weight: f32,
    pub(crate) condition: CombatCondition,
}

#[derive(Debug, Clone, Copy, Default, Reflect, FromReflect)]
pub(crate) struct CurrentMove {
    pub(crate) choreography: usize,
    pub(crate) move_: usize,
    pub(crate) start_transform: Transform,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Choreography {
    pub(crate) name: String,
    pub(crate) moves: Vec<Move>,
}

#[derive(
    Debug, Component, Clone, Copy, PartialEq, Default, Reflect, FromReflect, Serialize, Deserialize,
)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct CurrentMoveMetadata {
    pub(crate) start_transform: Transform,
    pub(crate) start_player_direction: Vec3,
    pub(crate) animation_duration: Option<f32>,
}

#[derive(
    Debug, Component, Clone, Copy, PartialEq, Default, Reflect, FromReflect, Serialize, Deserialize,
)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) enum EnemyCombatState {
    Deathblow,
    Vulnerable,
    #[default]
    OnGuard,
    HyperArmor,
    Dying,
}

#[derive(Debug, Component, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct AttackHitbox {
    pub(crate) active: bool,
    pub(crate) attack: Attack,
}

impl AttackHitbox {
    pub(crate) fn from_attack(attack: Attack) -> Self {
        Self {
            attack,
            ..default()
        }
    }
}

impl Default for AttackHitbox {
    fn default() -> Self {
        Self {
            active: true,
            attack: default(),
        }
    }
}

#[derive(
    Debug, Component, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize, Default,
)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct Attack {
    pub(crate) name: String,
    pub(crate) health_damage: f32,
    pub(crate) posture_damage: f32,
    pub(crate) knockback: f32,
}

impl Attack {
    pub(crate) fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self {
            name: name.into().to_string(),
            ..default()
        }
    }

    /// Source: <https://docs.google.com/spreadsheets/d/1mDiylVeazEJM3_M90zfJG-50nTfQgF6bRxlest9qg-g/edit#gid=0>  
    /// Could also use <https://www.reddit.com/r/Sekiro/comments/bk5c4d/damage_values_estimated_for_every_combat_art_and/> in the future
    pub(crate) fn with_health_damage_scaling_rest(self, health_damage: f32) -> Self {
        self.with_health_damage(health_damage)
            .with_posture_damage(health_damage * 0.375)
            .with_knockback(health_damage * 0.7)
    }

    pub(crate) fn with_health_damage(mut self, health_damage: f32) -> Self {
        self.health_damage = health_damage;
        self
    }

    pub(crate) fn with_posture_damage(mut self, posture_damage: f32) -> Self {
        self.posture_damage = posture_damage;
        self
    }

    pub(crate) fn with_knockback(mut self, knockback: f32) -> Self {
        self.knockback = knockback;
        self
    }
}

#[derive(
    Debug, Component, Clone, Copy, PartialEq, Default, Reflect, FromReflect, Serialize, Deserialize,
)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct Projectile;

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct HitboxParentModel;
