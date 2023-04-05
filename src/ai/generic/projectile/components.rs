use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Component, Clone, PartialEq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct SimpleProjectile {
    pub speed: f32,
    pub tracking: f32,
    pub current_lifetime: f32,
    pub max_lifetime: f32,
}
