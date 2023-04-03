use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::fmt::Debug;

impl Debug for dyn AttackFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AttackFn").finish()
    }
}

pub trait AttackFn: Send + Sync {
    fn call(&self, input: AttackFnInput) -> AttackFnOutput;
    fn clone_box<'a>(&self) -> Box<dyn AttackFn + 'a>
    where
        Self: 'a;
}
impl<F> AttackFn for F
where
    F: Fn(AttackFnInput) -> AttackFnOutput + Send + Sync + Clone,
{
    fn call(&self, input: AttackFnInput) -> AttackFnOutput {
        self(input)
    }

    fn clone_box<'a>(&self) -> Box<dyn AttackFn + 'a>
    where
        Self: 'a,
    {
        Box::new(self.clone())
    }
}

impl<'a> Clone for Box<dyn AttackFn + 'a> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}

#[derive(Debug, Clone, Default)]
pub struct AttackFnInput {
    pub time: f32,
    pub transform: Transform,
    pub player_direction: Vec3,
    pub start_player_direction: Vec3,
}

#[derive(Debug, Clone, Default)]
pub struct AttackFnOutput {
    pub collider: Collider,
    pub transform: Transform,
    pub damage: f32,
    pub knockback: f32,
}
