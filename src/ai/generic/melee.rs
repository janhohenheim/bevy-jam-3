use crate::combat::{Attack, AttackHitbox, MeleeAttackFn, MeleeAttackFnInput, MeleeAttackFnOutput};

pub(crate) fn whole_animation(attack: Attack) -> Box<dyn MeleeAttackFn> {
    Box::new(
        move |MeleeAttackFnInput { time: _time }: MeleeAttackFnInput| MeleeAttackFnOutput {
            melee_attack: AttackHitbox {
                active: true,
                attack: attack.clone(),
            },
        },
    )
}
