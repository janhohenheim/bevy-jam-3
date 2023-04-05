use crate::combat::{MeleeAttackFn, MeleeAttackFnInput, MeleeAttackFnOutput};
use bevy::prelude::*;

pub fn attack_at_keyframes(
    keyframes: Vec<(f32, Transform)>,
    damage: f32,
    knockback: f32,
) -> Box<dyn MeleeAttackFn> {
    Box::new(move |MeleeAttackFnInput { time }: MeleeAttackFnInput| {
        let times = keyframes.iter().map(|(time, _)| time).collect();

        let rotations = Keyframes::Rotation(
            keyframes
                .iter()
                .map(|(_, transform)| transform.rotation)
                .collect(),
        );
        let rotations = VariableCurve {
            keyframe_timestamps: times.clone(),
            keyframes: rotations,
        };

        let translations = Keyframes::Translation(
            keyframes
                .iter()
                .map(|(_, transform)| transform.translation)
                .collect(),
        );
        let translations = VariableCurve {
            keyframe_timestamps: times.clone(),
            keyframes: translations,
        };

        let scales = Keyframes::Scale(
            keyframes
                .iter()
                .map(|(_, transform)| transform.scale)
                .collect(),
        );
        let scales = VariableCurve {
            keyframe_timestamps: times,
            keyframes: scales,
        };

        let mut animation_clip = AnimationClip::default();
        let entity_path = EntityPath {
            parts: vec!["model".into()],
        };
        animation_clip.add_curve_to_path(entity_path.clone(), rotations);
        animation_clip.add_curve_to_path(entity_path.clone(), translations);
        animation_clip.add_curve_to_path(entity_path, scales);
        MeleeAttackFnOutput {
            animation_clip,
            damage,
            knockback,
        }
    })
}
