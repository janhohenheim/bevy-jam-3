use crate::combat::Constitution;
use crate::file_system_interaction::asset_loading::TextureAssets;
use crate::player_control::player_embodiment::Player;
use crate::GameState;
use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;

pub fn player_combat_ui_plugin(app: &mut App) {
    app.add_system(spawn_constitution_bars.in_schedule(OnEnter(GameState::Playing)));
}

fn spawn_constitution_bars(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            bottom: Val::Px(20.0),
                            ..Default::default()
                        },
                        size: Size::new(Val::Px(100.0), Val::Px(75.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        image: UiImage::new(textures.bar_border.clone()),
                        style: Style {
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        ..default()
                    });
                    parent.spawn(ImageBundle {
                        image: UiImage::new(textures.health_bar.clone()),
                        z_index: ZIndex::Global(1),
                        style: Style {
                            overflow: Overflow::Hidden,
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        ..default()
                    });
                });
        });
}

fn update_health_bar() {}
