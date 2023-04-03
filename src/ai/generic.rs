use crate::combat::{ForceFn, ForceFnInput};
use crate::movement::general_movement::Walking;

pub fn walk_towards_player(ground_acceleration: f32) -> Box<dyn ForceFn<Output = Walking>> {
    Box::new(
        move |ForceFnInput {
                  transform,
                  line_of_sight_path,
                  mut walking,
                  ..
              }: ForceFnInput| {
            let direction = (transform.translation - line_of_sight_path[0]).normalize();
            walking.direction = Some(direction);
            walking.ground_acceleration = ground_acceleration;
            walking
        },
    )
}
