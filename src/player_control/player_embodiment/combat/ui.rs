use crate::combat::Constitution;
use crate::file_system_interaction::asset_loading::TextureAssets;
use crate::player_control::player_embodiment::Player;
use crate::GameState;
use anyhow::Result;
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;

pub fn player_combat_ui_plugin(app: &mut App) {
    app.add_system(spawn_constitution_bars.in_schedule(OnEnter(GameState::Playing)))
        .add_systems((update_health_bar,).in_set(OnUpdate(GameState::Playing)));
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
                                bottom: Val::Px(50.0),
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
                                        ..default()
                                    },
                                    ..default()
                                },
                            ));
                            parent.spawn((
                                Name::new("Health bar fill image"),
                                ImageBundle {
                                    image: UiImage::new(textures.health_bar_fill.clone()),
                                    z_index: ZIndex::Global(1),
                                    style: Style {
                                        position_type: PositionType::Absolute,
                                        position: UiRect {
                                            left: Val::Px(-372.0),
                                            ..Default::default()
                                        },
                                        size: Size::new(
                                            Val::Px(MAX_HEALTH_BAR_WIDTH),
                                            Val::Px(93.),
                                        ),
                                        ..default()
                                    },
                                    ..default()
                                },
                            ));
                        });
                });
        });
}

const MAX_HEALTH_BAR_WIDTH: f32 = 742.0;

#[derive(Debug, Component, Clone, PartialEq)]
pub struct HealthBarLink(pub Entity);

#[derive(Debug, Component, Clone, PartialEq)]
pub struct PostureBarLink(pub Entity);

#[derive(Debug, Component, Clone, PartialEq)]
pub struct PostureBarParentLink(pub Entity);

#[sysfail(log(level = "error"))]
fn update_health_bar(
    players: Query<
        (
            &Constitution,
            &HealthBarLink,
            &PostureBarLink,
            &PostureBarParentLink,
        ),
        With<Player>,
    >,
    mut ui_bars: Query<&mut Style>,
    mut visibilities: Query<&mut Visibility>,
) -> Result<()> {
    for (constitution, health_bar_link, posture_bar_link, posture_bar_parent_link) in players.iter()
    {
        let mut health_bar = ui_bars.get_mut(health_bar_link.0)?;
        let health_fraction = constitution.health_fraction();
        let health_bar_width = MAX_HEALTH_BAR_WIDTH * health_fraction;
        health_bar.size.width = Val::Px(health_bar_width);

        let posture_fraction = constitution.posture_fraction();
        let posture_bar_visibility = visibilities.get_mut(posture_bar_link.0)?;
        if posture_fraction < 1e-5 {
            let mut posture_bar_parent = ui_bars.get_mut(posture_bar_parent_link.0)?;
        } else {
            let mut posture_bar = ui_bars.get_mut(posture_bar_link.0)?;
        }
    }
    Ok(())
}
