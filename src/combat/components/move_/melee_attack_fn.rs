use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::fmt::Debug;

impl Debug for dyn MeleeAttackFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MeleeAttackFn").finish()
    }
}

pub trait MeleeAttackFn: Send + Sync {
    fn call(&self, input: MeleeAttackFnInput) -> MeleeAttackFnOutput;
    fn clone_box<'a>(&self) -> Box<dyn MeleeAttackFn + 'a>
    where
        Self: 'a;
}
impl<F> MeleeAttackFn for F
where
    F: Fn(MeleeAttackFnInput) -> MeleeAttackFnOutput + Send + Sync + Clone,
{
    fn call(&self, input: MeleeAttackFnInput) -> MeleeAttackFnOutput {
        self(input)
    }

    fn clone_box<'a>(&self) -> Box<dyn MeleeAttackFn + 'a>
    where
        Self: 'a,
    {
        Box::new(self.clone())
    }
}

impl<'a> Clone for Box<dyn MeleeAttackFn + 'a> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}

#[derive(Debug, Clone, Default)]
pub struct MeleeAttackFnInput {
    pub time: f32,
}

#[derive(Debug, Clone, Default)]
pub struct MeleeAttackFnOutput {
    pub collider: Collider,
    pub transform: Transform,
    pub damage: f32,
    pub knockback: f32,
}
