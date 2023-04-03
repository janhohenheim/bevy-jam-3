use crate::combat::{MeleeAttack, MeleeAttackLink};
use bevy::prelude::*;

pub fn link_melee_attack(
    mut commands: Commands,
    melee_attacks: Query<(Entity, &Parent), Added<MeleeAttack>>,
) {
    for (entity, parent) in melee_attacks.iter() {
        let parent = parent.get();
        commands.entity(parent).insert(MeleeAttackLink(entity));
    }
}
