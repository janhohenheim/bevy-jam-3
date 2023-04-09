use crate::file_system_interaction::config::GameConfig;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::fmt::Debug;

impl Debug for dyn MotionFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ForceFn").finish()
    }
}

pub(crate) trait MotionFn: Send + Sync {
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
pub(crate) struct MotionFnInput {
    pub(crate) _time_in_move: f32,
    pub(crate) global_time: f32,
    pub(crate) dt: f32,
    pub(crate) _duration: Option<f32>,
    pub(crate) transform: Transform,
    pub(crate) _start_transform: Transform,
    pub(crate) player_direction: Vec3,
    pub(crate) start_player_direction: Vec3,
    pub(crate) _has_line_of_sight: bool,
    pub(crate) line_of_sight_direction: Vec3,
    pub(crate) mass: f32,
    pub(crate) velocity: Vec3,
    pub(crate) config: GameConfig,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct MotionFnOutput {
    pub(crate) force: ExternalForce,
    pub(crate) impulse: ExternalImpulse,
    pub(crate) rotation: Option<Quat>,
}
