use crate::combat::{Constitution, Enemy};
use crate::file_system_interaction::asset_loading::TextureAssets;
use crate::movement::general_movement::Height;
use crate::player_control::camera::IngameCamera;
use crate::GameState;
use anyhow::Result;
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;
use std::f32::consts::TAU;

pub(crate) fn enemy_combat_ui_plugin(app: &mut App) {
    app.add_system(create_billboard_assets.in_schedule(OnExit(GameState::Loading)))
        .add_systems(
            (
                spawn_constitution_bars,
                update_constitution_bars,
                update_billboards,
            )
                .chain()
                .in_set(OnUpdate(GameState::Playing)),
        );
}

#[derive(Debug, Clone, PartialEq, Resource, Reflect, FromReflect, Default)]
#[reflect(Resource)]
pub(crate) struct BillboardAssets {
    pub(crate) bar_border: Handle<StandardMaterial>,
    pub(crate) health_bar_fill: Handle<StandardMaterial>,
    pub(crate) posture_bar_fill: Handle<StandardMaterial>,
    pub(crate) posture_bar_top: Handle<StandardMaterial>,
    pub(crate) health_bar_mesh: Handle<Mesh>,
    pub(crate) posture_bar_mesh: Handle<Mesh>,
}

const BAR_WIDTH: f32 = 0.4;
const HEALTH_BAR_HEIGHT: f32 = 0.05;
const POSTURE_BAR_HEIGHT: f32 = 0.03;

fn create_billboard_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    textures: Res<TextureAssets>,
) {
    commands.insert_resource(BillboardAssets {
        bar_border: materials.add(create_billboard_material(&textures.bar_border)),
        health_bar_fill: materials.add(create_billboard_material(&textures.health_bar_fill)),
        posture_bar_fill: materials.add(create_billboard_material(&textures.posture_bar_fill)),
        posture_bar_top: materials.add(create_billboard_material(&textures.posture_bar_top)),
        health_bar_mesh: meshes
            .add(shape::Quad::new(Vec2::new(BAR_WIDTH, HEALTH_BAR_HEIGHT)).into()),
        posture_bar_mesh: meshes
            .add(shape::Quad::new(Vec2::new(BAR_WIDTH, POSTURE_BAR_HEIGHT)).into()),
    });
}

fn create_billboard_material(texture: &Handle<Image>) -> StandardMaterial {
    StandardMaterial {
        base_color_texture: Some(texture.clone()),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        perceptual_roughness: 1.0,
        reflectance: 0.0,
        ..default()
    }
}

fn spawn_constitution_bars(
    mut commands: Commands,
    enemies: Query<(Entity, &Height), Added<Enemy>>,
    billboard_assets: Res<BillboardAssets>,
) {
    for (entity, height) in enemies.iter() {
        let health_bar_fill = commands
            .spawn((
                Name::new("Health bar fill"),
                PbrBundle {
                    transform: Transform::from_translation(Vec3::new(0., 0.0, 1e-2)),
                    material: billboard_assets.health_bar_fill.clone(),
                    mesh: billboard_assets.health_bar_mesh.clone().into(),
                    ..default()
                },
            ))
            .id();

        let health_bar_y = height.half() + POSTURE_BAR_HEIGHT + HEALTH_BAR_HEIGHT / 2. - 0.2;

        commands
            .spawn((
                Name::new("Health bar"),
                SpatialBundle::default(),
                Billboard {
                    follow_target: entity,
                    offset: Vec3::new(0., health_bar_y, 0.),
                },
                NotShadowCaster,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Name::new("Health bar border"),
                    PbrBundle {
                        transform: Transform::from_translation(Vec3::new(0., 0.0, -1e-2)),
                        material: billboard_assets.bar_border.clone(),
                        mesh: billboard_assets.health_bar_mesh.clone(),
                        ..default()
                    },
                ));
            })
            .add_child(health_bar_fill);

        let posture_bar_fill = commands
            .spawn((
                Name::new("Posture bar fill"),
                PbrBundle {
                    transform: Transform::from_translation(Vec3::new(0., 0.0, 1e-2)),
                    material: billboard_assets.posture_bar_fill.clone(),
                    mesh: billboard_assets.posture_bar_mesh.clone().into(),
                    ..default()
                },
            ))
            .id();

        let posture_bar_y = height.half() + POSTURE_BAR_HEIGHT / 2. - 0.22;
        let posture_bar = commands
            .spawn((
                Billboard {
                    follow_target: entity,
                    offset: Vec3::new(0., posture_bar_y, 0.),
                },
                Name::new("Posture bar"),
                SpatialBundle::default(),
                NotShadowCaster,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Name::new("Posture bar border"),
                    PbrBundle {
                        material: billboard_assets.bar_border.clone(),
                        mesh: billboard_assets.posture_bar_mesh.clone(),
                        transform: Transform::from_translation(Vec3::new(0., 0.0, -1e-2)),
                        ..default()
                    },
                ));
                parent.spawn((
                    Name::new("Posture bar top decoration"),
                    PbrBundle {
                        transform: Transform::from_translation(Vec3::new(0., 0.0, 3e-2)),
                        material: billboard_assets.posture_bar_top.clone(),
                        mesh: billboard_assets.posture_bar_mesh.clone().into(),
                        ..default()
                    },
                ));
            })
            .add_child(posture_bar_fill)
            .id();

        commands.entity(entity).insert((
            HealthBarFillLink(health_bar_fill),
            PostureBarFillLink(posture_bar_fill),
            PostureBarParentLink(posture_bar),
        ));
    }
}

#[derive(Debug, Component, Clone, PartialEq, Deref, DerefMut)]
pub(crate) struct HealthBarFillLink(Entity);

#[derive(Debug, Component, Clone, PartialEq, Deref, DerefMut)]
pub(crate) struct PostureBarFillLink(Entity);

#[derive(Debug, Component, Clone, PartialEq, Deref, DerefMut)]
pub(crate) struct PostureBarParentLink(Entity);

#[derive(Debug, Component, Clone, PartialEq)]
pub(crate) struct Billboard {
    follow_target: Entity,
    offset: Vec3,
}

#[sysfail(log(level = "error"))]
fn update_constitution_bars(
    enemies: Query<(
        &Constitution,
        &HealthBarFillLink,
        &PostureBarFillLink,
        &PostureBarParentLink,
    )>,
    mut transforms: Query<&mut Transform>,
    mut visibilities: Query<&mut Visibility>,
) -> Result<()> {
    for (constitution, health_bar_fill_link, posture_bar_fill_link, posture_bar_parent_link) in
        enemies.iter()
    {
        let health_fraction = constitution.health_fraction();
        let mut health_bar_fill_transform = transforms.get_mut(health_bar_fill_link.0)?;
        health_bar_fill_transform.scale.x = health_fraction;
        health_bar_fill_transform.translation.x =
            -BAR_WIDTH / 2. + BAR_WIDTH * health_fraction / 2. + 0.01;

        let posture_fraction = constitution.posture_fraction();
        let mut posture_bar_visibility = visibilities.get_mut(posture_bar_parent_link.0)?;
        if posture_fraction < 1e-5 {
            *posture_bar_visibility = Visibility::Hidden;
        } else {
            let mut posture_bar_fill_transform = transforms.get_mut(posture_bar_fill_link.0)?;
            posture_bar_fill_transform.scale.x = posture_fraction;
            *posture_bar_visibility = Visibility::Inherited;
        }
    }
    Ok(())
}

fn update_billboards(
    mut commands: Commands,
    mut billboards: Query<(Entity, &mut Transform, &Billboard), Without<IngameCamera>>,
    parents: Query<&Transform, (Without<Billboard>, Without<IngameCamera>)>,
    mut cameras: Query<&Transform, With<IngameCamera>>,
) {
    for (entity, mut transform, billboard) in billboards.iter_mut() {
        if let Ok(parent_transform) = parents.get(billboard.follow_target) {
            for camera_transform in cameras.iter_mut() {
                transform.translation = parent_transform.translation + billboard.offset;
                transform.look_at(camera_transform.translation, Vec3::Y);
                transform.rotation *= Quat::from_rotation_y(TAU / 2.);
            }
        } else {
            commands.entity(entity).despawn_recursive();
        }
    }
}
