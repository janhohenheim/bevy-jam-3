use crate::combat::Constitution;
use crate::file_system_interaction::asset_loading::TextureAssets;
use crate::player_control::player_embodiment::Player;
use crate::GameState;
use anyhow::Result;
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;

pub fn player_combat_ui_plugin(app: &mut App) {
    app.add_system(spawn_constitution_bars.in_schedule(OnEnter(GameState::Playing)))
        .add_systems((update_constitution_bars,).in_set(OnUpdate(GameState::Playing)));
}

fn spawn_constitution_bars(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn((
            Name::new("Player UI root node"),
            NodeBundle {
                style: Style {
                    size: Size::width(Val::Percent(100.0)),
                    align_items: AlignItems::End,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Name::new("Constitution root"),
                    NodeBundle {
                        style: Style {
                            position: UiRect {
                                bottom: Val::Px(12.0),
                                ..Default::default()
                            },
                            size: Size::new(Val::Px(100.0), Val::Px(75.0)),
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            Name::new("Health bar root"),
                            NodeBundle {
                                style: Style {
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                ..default()
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Name::new("Health bar border image"),
                                ImageBundle {
                                    image: UiImage::new(textures.bar_border.clone()),
                                    style: Style {
                                        position_type: PositionType::Absolute,
                                        size: Size::new(Val::Px(BAR_WIDTH), Val::Px(HEALTH_HEIGHT)),
                                        ..default()
                                    },
                                    ..default()
                                },
                            ));
                            parent.spawn((
                                Name::new("Health bar fill image"),
                                HealthBarFill,
                                ImageBundle {
                                    image: UiImage::new(textures.health_bar_fill.clone()),
                                    z_index: ZIndex::Global(1),
                                    style: Style {
                                        position_type: PositionType::Absolute,
                                        position: UiRect {
                                            left: Val::Px(-372.0),
                                            ..Default::default()
                                        },
                                        size: Size::new(Val::Px(BAR_WIDTH), Val::Px(HEALTH_HEIGHT)),
                                        ..default()
                                    },
                                    ..default()
                                },
                            ));
                        });
                    parent
                        .spawn((
                            Name::new("Posture bar root"),
                            PostureBarParent,
                            NodeBundle {
                                style: Style {
                                    justify_content: JustifyContent::Center,
                                    position: UiRect {
                                        bottom: Val::Px(42.),
                                        ..Default::default()
                                    },
                                    ..default()
                                },
                                ..default()
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Name::new("Posture bar border image"),
                                ImageBundle {
                                    image: UiImage::new(textures.bar_border.clone()),
                                    style: Style {
                                        position_type: PositionType::Absolute,
                                        size: Size::new(
                                            Val::Px(BAR_WIDTH),
                                            Val::Px(POSTURE_HEIGHT),
                                        ),
                                        ..default()
                                    },
                                    ..default()
                                },
                            ));
                            parent.spawn((
                                Name::new("Posture bar fill image"),
                                PostureBarFill,
                                ImageBundle {
                                    image: UiImage::new(textures.posture_bar_fill.clone()),
                                    z_index: ZIndex::Global(1),
                                    style: Style {
                                        position_type: PositionType::Absolute,
                                        size: Size::new(
                                            Val::Px(BAR_WIDTH),
                                            Val::Px(POSTURE_HEIGHT),
                                        ),
                                        ..default()
                                    },
                                    ..default()
                                },
                            ));
                            parent.spawn((
                                Name::new("Posture bar top decoration"),
                                ImageBundle {
                                    image: UiImage::new(textures.posture_bar_top.clone()),
                                    z_index: ZIndex::Global(2),
                                    style: Style {
                                        position_type: PositionType::Absolute,
                                        size: Size::new(Val::Px(337.0), Val::Px(POSTURE_HEIGHT)),
                                        ..default()
                                    },
                                    ..default()
                                },
                            ));
                        });
                });
        });
}

const BAR_WIDTH: f32 = 742.0;
const HEALTH_HEIGHT: f32 = 50.0;
const POSTURE_HEIGHT: f32 = 30.0;

#[derive(Debug, Component, Clone, PartialEq)]
pub struct HealthBarFill;

#[derive(Debug, Component, Clone, PartialEq)]
pub struct PostureBarFill;

#[derive(Debug, Component, Clone, PartialEq)]
pub struct PostureBarParent;

#[sysfail(log(level = "error"))]
fn update_constitution_bars(
    players: Query<(&Constitution,), With<Player>>,
    mut health_bar_fills: Query<&mut Style, (With<HealthBarFill>, Without<PostureBarFill>)>,
    mut posture_bar_fills: Query<&mut Style, (With<PostureBarFill>, Without<HealthBarFill>)>,
    mut posture_bar_parents: Query<&mut Visibility, With<PostureBarParent>>,
) -> Result<()> {
    for (constitution,) in players.iter() {
        for mut health_bar_fill in health_bar_fills.iter_mut() {
            for mut posture_bar_fill in posture_bar_fills.iter_mut() {
                for mut posture_bar_visibility in posture_bar_parents.iter_mut() {
                    let health_fraction = constitution.health_fraction();
                    let health_bar_width = BAR_WIDTH * health_fraction;
                    health_bar_fill.size.width = Val::Px(health_bar_width);

                    let posture_fraction = constitution.posture_fraction();
                    if posture_fraction < 1e-5 {
                        *posture_bar_visibility = Visibility::Hidden;
                    } else {
                        posture_bar_fill.size.width = Val::Px(BAR_WIDTH * posture_fraction);
                        *posture_bar_visibility = Visibility::Inherited;
                    }
                }
            }
        }
    }
    Ok(())
}
