use crate::combat::Constitution;
use crate::file_system_interaction::asset_loading::TextureAssets;
use crate::player_control::player_embodiment::Player;
use crate::GameState;
use anyhow::Result;
use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;
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
                                bottom: Val::Px(20.0),
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
                            image: UiImage::new(textures.health_bar.clone()),
                            z_index: ZIndex::Global(1),
                            style: Style {
                                position_type: PositionType::Absolute,
                                size: Size::new(Val::Px(50.0), Val::Px(75.)),
                                ..default()
                            },
                            ..default()
                        },
                    ));
                });
        });
}

#[derive(Debug, Component, Clone, PartialEq)]
pub struct HealthBarLink(pub Entity);

#[derive(Debug, Component, Clone, PartialEq)]
pub struct PostureBarLink(pub Entity);

#[sysfail(log(level = "error"))]
fn update_health_bar(
    players: Query<(&Constitution, &HealthBarLink, &PostureBarLink), With<Player>>,
    mut ui_bars: Query<&mut Style>,
) -> Result<()> {
    for (constitution, health_bar_link, posture_bar_link) in players.iter() {
        let mut health_bar = ui_bars.get_mut(health_bar_link.0)?;
        let mut posture_bar = ui_bars.get_mut(posture_bar_link.0)?;
    }
    Ok(())
}
