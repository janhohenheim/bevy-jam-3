use crate::combat::{TranslationFn, TranslationFnInput};
use bevy::prelude::*;

pub fn walk_towards_player(speed: f32) -> Box<dyn TranslationFn<Output = Vec3>> {
    Box::new(
        move |TranslationFnInput {
                  time,
                  transform,
                  start_transform,
                  player_direction,
                  start_player_direction,
                  has_line_of_sight,
                  line_of_sight_path,
              }: TranslationFnInput| { Vec3::new(speed, 0.0, 0.0) },
    )
}
