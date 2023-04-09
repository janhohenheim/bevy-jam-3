use crate::combat::{Combatant, Constitution};
use crate::file_system_interaction::asset_loading::TextureAssets;
use crate::movement::general_movement::Height;
use crate::GameState;
use anyhow::Result;
use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;
use bevy_mod_sysfail::macros::*;

pub fn enemy_combat_ui_plugin(app: &mut App) {
    app.add_plugin(BillboardPlugin)
        .add_system(create_billboard_assets.in_schedule(OnExit(GameState::Loading)))
        .add_systems(
            (spawn_constitution_bars, update_constitution_bars)
                .chain()
                .in_set(OnUpdate(GameState::Playing)),
        );
}

#[derive(Debug, Clone, PartialEq, Resource, Reflect, FromReflect, Default)]
#[reflect(Resource)]
pub struct BillboardAssets {
    pub bar_border: Handle<BillboardTexture>,
    pub health_bar_fill: Handle<BillboardTexture>,
    pub posture_bar_fill: Handle<BillboardTexture>,
    pub posture_bar_top: Handle<BillboardTexture>,
    pub health_bar_mesh: Handle<Mesh>,
    pub posture_bar_mesh: Handle<Mesh>,
}

const BAR_WIDTH: f32 = 0.4;
const HEALTH_BAR_HEIGHT: f32 = 0.1;
const POSTURE_BAR_HEIGHT: f32 = 0.05;

fn create_billboard_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut billboard_textures: ResMut<Assets<BillboardTexture>>,
    textures: Res<TextureAssets>,
) {
    commands.insert_resource(BillboardAssets {
        bar_border: billboard_textures.add(BillboardTexture::Single(textures.bar_border.clone())),
        health_bar_fill: billboard_textures
            .add(BillboardTexture::Single(textures.health_bar_fill.clone())),
        posture_bar_fill: billboard_textures
            .add(BillboardTexture::Single(textures.posture_bar_fill.clone())),
        posture_bar_top: billboard_textures
            .add(BillboardTexture::Single(textures.posture_bar_top.clone())),
        health_bar_mesh: meshes
            .add(shape::Quad::new(Vec2::new(BAR_WIDTH, HEALTH_BAR_HEIGHT)).into()),
        posture_bar_mesh: meshes
            .add(shape::Quad::new(Vec2::new(BAR_WIDTH, POSTURE_BAR_HEIGHT)).into()),
    });
}

fn spawn_constitution_bars(
    mut commands: Commands,
    enemies: Query<(Entity, &Height), Added<Combatant>>,
    billboard_assets: Res<BillboardAssets>,
) {
    for (entity, height) in enemies.iter() {
        let health_bar_fill = commands
            .spawn((
                Name::new("Health bar fill"),
                BillboardTextureBundle {
                    transform: Transform::from_translation(Vec3::new(0., 0.0, -1e-3)),
                    texture: billboard_assets.health_bar_fill.clone(),
                    mesh: billboard_assets.health_bar_mesh.clone().into(),
                    ..default()
                },
            ))
            .id();

        let health_bar_y = height.half() + 0.5;

        commands.entity(entity).with_children(|parent| {
            parent
                .spawn((
                    Name::new("Health bar"),
                    SpatialBundle::from_transform(Transform::from_xyz(0., health_bar_y, 0.)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Name::new("Health bar border"),
                        BillboardTextureBundle {
                            texture: billboard_assets.bar_border.clone(),
                            mesh: billboard_assets.health_bar_mesh.clone().into(),
                            ..default()
                        },
                    ));
                })
                .add_child(health_bar_fill);
        });

        let posture_bar_fill = commands
            .spawn((
                Name::new("Posture bar fill"),
                BillboardTextureBundle {
                    transform: Transform::from_translation(Vec3::new(0., 0.0, -1e-3)),
                    texture: billboard_assets.posture_bar_fill.clone(),
                    mesh: billboard_assets.posture_bar_mesh.clone().into(),
                    ..default()
                },
            ))
            .id();

        let posture_bar_y = health_bar_y - HEALTH_BAR_HEIGHT / 2. - 0.1;
        let posture_bar = commands
            .spawn((
                Name::new("Posture bar"),
                SpatialBundle::from_transform(Transform::from_xyz(0., posture_bar_y, 0.)),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Name::new("Posture bar border"),
                    BillboardTextureBundle {
                        texture: billboard_assets.bar_border.clone(),
                        mesh: billboard_assets.posture_bar_mesh.clone().into(),
                        ..default()
                    },
                ));
                parent.spawn((
                    Name::new("Posture bar top decoration"),
                    BillboardTextureBundle {
                        transform: Transform::from_translation(Vec3::new(0., 0.0, -2e-3)),
                        texture: billboard_assets.posture_bar_top.clone(),
                        mesh: billboard_assets.posture_bar_mesh.clone().into(),
                        ..default()
                    },
                ));
            })
            .add_child(posture_bar_fill)
            .id();

        commands.entity(entity).add_child(posture_bar);

        commands.entity(entity).insert((
            HealthBarFillLink(health_bar_fill),
            PostureBarFillLink(posture_bar_fill),
            PostureBarParentLink(posture_bar),
        ));
    }
}

#[derive(Debug, Component, Clone, PartialEq, Deref, DerefMut)]
pub struct HealthBarFillLink(Entity);

#[derive(Debug, Component, Clone, PartialEq, Deref, DerefMut)]
pub struct PostureBarFillLink(Entity);

#[derive(Debug, Component, Clone, PartialEq, Deref, DerefMut)]
pub struct PostureBarParentLink(Entity);

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
            -BAR_WIDTH / 2. + BAR_WIDTH * health_fraction / 2.;

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
