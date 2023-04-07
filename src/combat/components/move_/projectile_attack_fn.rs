use crate::combat::AttackHitbox;
use bevy::prelude::*;
use spew::prelude::*;
use std::fmt::Debug;

impl Debug for dyn ProjectileAttackFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProjectileAttackFn").finish()
    }
}

pub trait ProjectileAttackFn: Send + Sync {
    fn call(&self, input: ProjectileAttackFnInput) -> ProjectileAttackFnOutput;
    fn clone_box<'a>(&self) -> Box<dyn ProjectileAttackFn + 'a>
    where
        Self: 'a;
}
impl<F> ProjectileAttackFn for F
where
    F: Fn(ProjectileAttackFnInput) -> ProjectileAttackFnOutput + Send + Sync + Clone,
{
    fn call(&self, input: ProjectileAttackFnInput) -> ProjectileAttackFnOutput {
        self(input)
    }

    fn clone_box<'a>(&self) -> Box<dyn ProjectileAttackFn + 'a>
    where
        Self: 'a,
    {
        Box::new(self.clone())
    }
}

impl<'a> Clone for Box<dyn ProjectileAttackFn + 'a> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}

#[derive(Debug, Clone)]
pub struct ProjectileAttackFnInput {
    pub time: f32,
    pub spawner: Entity,
}

#[derive(Debug, Clone, Default)]
pub struct ProjectileAttackFnOutput {
    pub spawn_events: Vec<SpawnEvent<ProjectileKind, (Entity, ProjectileSpawnInput)>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ProjectileKind {
    Simple,
}

#[derive(Debug, Clone, Reflect, FromReflect, Default)]
pub struct ProjectileSpawnInput {
    pub(crate) model: Handle<Scene>,
    pub(crate) attack: AttackHitbox,
    pub(crate) speed: f32,
    /// 0-1
    pub(crate) tracking: f32,
    pub(crate) max_lifetime: f32,
}
