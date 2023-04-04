use crate::file_system_interaction::config::GameConfig;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::fmt::Debug;

impl Debug for dyn ForceFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ForceFn").finish()
    }
}

pub trait ForceFn: Send + Sync {
    fn call(&self, input: ForceFnInput) -> ForceFnOutput;
    fn clone_box<'a>(&self) -> Box<dyn ForceFn + 'a>
    where
        Self: 'a;
}
impl<F> ForceFn for F
where
    F: Fn(ForceFnInput) -> ForceFnOutput + Send + Sync + Clone,
{
    fn call(&self, input: ForceFnInput) -> ForceFnOutput {
        self(input)
    }

    fn clone_box<'a>(&self) -> Box<dyn ForceFn + 'a>
    where
        Self: 'a,
    {
        Box::new(self.clone())
    }
}

impl<'a> Clone for Box<dyn ForceFn + 'a> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ForceFnInput {
    pub time: f32,
    pub dt: f32,
    pub transform: Transform,
    pub start_transform: Transform,
    pub player_direction: Vec3,
    pub start_player_direction: Vec3,
    pub has_line_of_sight: bool,
    pub line_of_sight_direction: Vec3,
    pub mass: f32,
    pub velocity: Vec3,
    pub config: GameConfig,
}

#[derive(Debug, Clone, Default)]
pub struct ForceFnOutput {
    pub force: ExternalForce,
    pub rotation: Option<Quat>,
}
