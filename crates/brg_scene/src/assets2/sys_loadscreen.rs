use bevy::color::palettes::tailwind::{LIME_300, ROSE_600};
use bevy::prelude::*;

use super::cmp::{CmpErrorText, CmpLoadingProgressBar, CmpLoadingText};
use super::sup_loader::SupAssetLoader;
use crate::prelude::GameState;

pub fn sys_spawn_loading_screen(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/RubikRegular.ttf");

    cmd.spawn((StateScoped(GameState::Loading), Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        align_content: AlignContent::Stretch,
        justify_content: JustifyContent::FlexEnd,
        ..default()
    }))
    .with_children(|parent| {
        parent
            .spawn((Node {
                width: Val::Px(400.),
                height: Val::Percent(100.0),
                padding: UiRect::axes(Val::Px(20.), Val::Px(0.)),
                flex_direction: FlexDirection::Column,
                align_content: AlignContent::Stretch,
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },))
            .with_children(|parent| {
                parent
                    .spawn((
                        Node {
                            width: Val::Percent(100.),
                            height: Val::Px(20.0),
                            flex_direction: FlexDirection::Column,
                            align_content: AlignContent::Start,
                            justify_content: JustifyContent::Center,
                            padding: UiRect::all(Val::Px(4.)),
                            border: UiRect::all(Val::Px(1.)),
                            ..default()
                        },
                        BorderColor(Color::srgb(0.2, 0.2, 0.2)),
                        BackgroundColor(Color::srgb(0.05, 0.05, 0.05)),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            CmpLoadingProgressBar,
                            Node {
                                width: Val::Percent(0.),
                                height: Val::Percent(100.),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.6, 0.6, 0.6)),
                        ));
                    });
            });

        parent
            .spawn((Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::axes(Val::Px(20.), Val::Px(20.)),
                ..default()
            },))
            .with_children(|parent| {
                parent.spawn((
                    CmpLoadingText,
                    Text::new("Loading..."),
                    TextFont {
                        font: font.clone(),
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor::WHITE,
                ));
                parent.spawn((
                    CmpErrorText,
                    Text::new(""),
                    TextFont {
                        font: font.clone(),
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(ROSE_600.into()),
                ));
            });
    });
}

pub fn sys_load_assets(mut s: SupAssetLoader) {
    match s.load_all() {
        Err(err) => {
            println!("Error loading assets: {}", err);
            panic!("can`t continue to starting game")
        }
        _ => {}
    }
}

pub fn sys_check_loading_status(
    mut s: SupAssetLoader,
    mut q_text: Query<&mut Text, With<CmpLoadingText>>,
    mut q_error: Query<&mut Text, (With<CmpErrorText>, Without<CmpLoadingText>)>,
    mut q_progress_bar: Query<(&mut Node, &mut BackgroundColor), With<CmpLoadingProgressBar>>,
) {
    let status = s.update_loading_status();

    // update text with current loading asset
    {
        for mut text in &mut q_text {
            if text.0 != status.last_info_title {
                text.0 = format!("Loading ({})", status.last_info_title);
            }
        }
    }

    // update last loading error text
    {
        if let Some(err) = status.last_info_error {
            for mut text in &mut q_error {
                if text.0 != err {
                    text.0 = err.clone();
                }
            }
        }
    }

    // update progress bar
    {
        let done = (status.cnt_loaded + status.cnt_failed) as f32;
        let percent = done / status.cnt_total as f32;

        for (mut node, mut bg) in &mut q_progress_bar {
            node.width = Val::Percent(percent * 100.0);

            if status.cnt_loaded == status.cnt_total {
                bg.0 = LIME_300.into();
            }

            if status.cnt_failed > 0 {
                bg.0 = ROSE_600.into();
            }
        }
    }
}
