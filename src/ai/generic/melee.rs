use crate::combat::{AttackHitbox, MeleeAttackFn, MeleeAttackFnInput, MeleeAttackFnOutput};

pub fn whole_animation(damage: f32, knockback: f32) -> Box<dyn MeleeAttackFn> {
    Box::new(
        move |MeleeAttackFnInput { time: _time }: MeleeAttackFnInput| MeleeAttackFnOutput {
            melee_attack: AttackHitbox {
                active: true,
                damage,
                knockback,
            },
        },
    )
}
