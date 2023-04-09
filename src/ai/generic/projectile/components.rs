use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Component, Clone, PartialEq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct SimpleProjectile {
    pub(crate) speed: f32,
    pub(crate) tracking: f32,
    pub(crate) current_lifetime: f32,
    pub(crate) max_lifetime: f32,
}
