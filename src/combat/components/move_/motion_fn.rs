use crate::file_system_interaction::config::GameConfig;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::fmt::Debug;

impl Debug for dyn MotionFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ForceFn").finish()
    }
}

pub trait MotionFn: Send + Sync {
    fn call(&self, input: MotionFnInput) -> MotionFnOutput;
    fn clone_box<'a>(&self) -> Box<dyn MotionFn + 'a>
    where
        Self: 'a;
}
impl<F> MotionFn for F
where
    F: Fn(MotionFnInput) -> MotionFnOutput + Send + Sync + Clone,
{
    fn call(&self, input: MotionFnInput) -> MotionFnOutput {
        self(input)
    }

    fn clone_box<'a>(&self) -> Box<dyn MotionFn + 'a>
    where
        Self: 'a,
    {
        Box::new(self.clone())
    }
}

impl<'a> Clone for Box<dyn MotionFn + 'a> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}

#[derive(Debug, Clone, Default)]
pub struct MotionFnInput {
    pub time: f32,
    pub dt: f32,
    pub duration: Option<f32>,
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
pub struct MotionFnOutput {
    pub force: ExternalForce,
    pub impulse: ExternalImpulse,
    pub rotation: Option<Quat>,
}
