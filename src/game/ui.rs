use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use super::dice::*;
use super::asset_loader::*;
use super::roll_dice::*;

#[derive(Debug)]
pub enum GameButtonType {
    Add,
    Remove,
    Roll,
    Mute,
    Motion
}

#[derive(Component, Debug)]
pub struct GameButton{
    pub button_type: GameButtonType
}

#[derive(Component)]
pub struct UiDiceNum;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_ui)
            .add_systems(Update, dice_number_check_system)
            .add_systems(Update, button_system);
    }
}

fn setup_ui(
    mut commands: Commands,
) {
//root
commands
    .spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        // background_color: BackgroundColor(Color::RED),
        ..default()
    })
    .with_children(|parent| {

        //위
        parent
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(50.),
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Start,
                    
                    ..default()
                },
                // background_color: BackgroundColor(Color::WHITE),
                ..default()
            })
            .with_children(|parent| {
                
                //left option
                parent.spawn(
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            ..default()
                        },
                        ..default()
                    }
                )
                .with_children(|parent| {
                    //mute btn
                    parent
                        .spawn((ButtonBundle {
                            style: Style {
                                width: Val::Percent(30.0),
                                height: Val::Percent(10.0),
                                border: UiRect::all(Val::Px(1.0)),
                                // // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: BackgroundColor(Color::YELLOW_GREEN),
                            border_color: BorderColor(Color::BLACK),
                            // image: UiImage::default().with_color(NORMAL_BUTTON),
                            ..default()
                        }, GameButton{ button_type: GameButtonType::Mute }))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_sections(vec![
                                    TextSection::new(
                                        "Mute-",
                                        TextStyle {
                                            ..default()
                                        }
                                    ),
                                    TextSection::new(
                                        "X",
                                        TextStyle {
                                            ..default()
                                        }
                                    )
                                ])
                            );
                        });

                    //motion btn
                    parent
                        .spawn((ButtonBundle {
                            style: Style {
                                width: Val::Percent(30.0),
                                height: Val::Percent(10.0),
                                border: UiRect::all(Val::Px(1.0)),
                                // // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: BackgroundColor(Color::PINK),
                            border_color: BorderColor(Color::BLACK),
                            // image: UiImage::default().with_color(NORMAL_BUTTON),
                            ..default()
                        }, GameButton{ button_type: GameButtonType::Motion }))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_sections(vec![
                                    TextSection::new(
                                        "Motion-",
                                        TextStyle {
                                            ..default()
                                        }
                                    ),
                                    TextSection::new(
                                        "O",
                                        TextStyle {
                                            ..default()
                                        }
                                    )
                                ])
                            );
                        });
                });

                // absolute 안하면 자릿수 늘어날때마다 왼쪽 버튼들의 사이즈가 줄어듬
                // dice number
                parent.spawn(
                    (
                        TextBundle::from_section(
                            "-",
                            TextStyle {
                                font_size: 80.,
                                color: Color::rgb(0.396, 0.278, 0.129),
                                ..default()
                            }
                        )
                        .with_style(Style {
                            position_type: PositionType::Absolute,
                            right: Val::Px(0.),
                            top: Val::Px(0.),
                            ..default()
                        }),
                        UiDiceNum
                    )
                );
            });

        //아래
        parent
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(50.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::End,

                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn((ButtonBundle {
                        style: Style {
                            width: Val::Percent(30.0),
                            height: Val::Percent(10.0),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        // image: UiImage::default().with_color(NORMAL_BUTTON),
                        ..default()
                    }, GameButton{ button_type: GameButtonType::Add }))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "ADD",
                            TextStyle {
                                // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 20.0,
                                color: Color::BLACK,
                                ..default()
                            },
                        ));
                    });
                parent
                    .spawn((ButtonBundle {
                        style: Style {
                            width: Val::Percent(30.0),
                            height: Val::Percent(10.0),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        // image: UiImage::default().with_color(NORMAL_BUTTON),
                        ..default()
                        }, GameButton{ button_type: GameButtonType::Remove }))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "REMOVE",
                            TextStyle {
                                // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 20.0,
                                color: Color::BLACK,
                                ..default()
                            },
                        ));
                    });

                parent
                    .spawn((ButtonBundle {
                        style: Style {
                            width: Val::Percent(30.0),
                            height: Val::Percent(10.0),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        // image: UiImage::default().with_color(NORMAL_BUTTON),
                        ..default()
                    }, GameButton{ button_type: GameButtonType::Roll }))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "ROLL",
                            TextStyle {
                                // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 20.0,
                                color: Color::BLACK,
                                ..default()
                            },
                        ));
                    });
            });
    });
}

/*
    6[x:  0, y:  ?, z:  0]0~1, 179~180
    1[x:  0, y:  ?, z:  0]

    4[x:  0, y:  ?, z: 90]
    3[x:  0, y:  ?, z: 90]

    2[x: 90, y:  0, z:  ?]
    5[x: 90, y:  0, z:  ?]
*/
fn dice_number_check_system(
    q_dices: Query<(&Transform, &Children), With<Dice>>,
    q_dice_num: Query<(&DiceNum, &GlobalTransform)>,
    mut q_num_text: Query<&mut Text, With<UiDiceNum>>
    
) {
    let mut total_num = 0;
    let mut dice_count = 0;
    
    for (_trans, child) in q_dices.iter() {
        dice_count += 1;
        let mut highest_num: i32 = 0;
        let mut before_y_val = -10000.;
        for &nums in child.iter() {
            if let Ok((num, tr)) = q_dice_num.get(nums) {
                if num.dice_num_type == DiceNumType::One {
                    let y = tr.translation().y;
                    if y > before_y_val {
                        before_y_val = y;
                        highest_num = 1;
                    }
                }
                if num.dice_num_type == DiceNumType::Two {
                    let y = tr.translation().y;
                    if y > before_y_val {
                        before_y_val = y;
                        highest_num = 2;
                    }
                }
                if num.dice_num_type == DiceNumType::Three {
                    let y = tr.translation().y;
                    if y > before_y_val {
                        before_y_val = y;
                        highest_num = 3;
                    }
                }
                if num.dice_num_type == DiceNumType::Four {
                    let y = tr.translation().y;
                    if y > before_y_val {
                        before_y_val = y;
                        highest_num = 4;
                    }
                }
                if num.dice_num_type == DiceNumType::Five {
                    let y = tr.translation().y;
                    if y > before_y_val {
                        before_y_val = y;
                        highest_num = 5;
                    }
                }
                if num.dice_num_type == DiceNumType::Six { 
                    let y = tr.translation().y;
                    if y > before_y_val {
                        before_y_val = y;
                        highest_num = 6;
                    }
                }
            }
        }
        total_num += highest_num;
    }
    let mut text = q_num_text.single_mut();
    text.sections[0].value = format!("{}-{}", dice_count, total_num);
}

// 버튼 시스템
fn button_system(
    mut btn_query: Query<(&GameButton, &Interaction, &mut BackgroundColor, &Children), (Changed<Interaction>, With<GameButton>)>,
    mut commands: Commands,
    mut ext_impulses: Query<&mut ExternalImpulse>,
    despawn_dices: Query<Entity, With<Dice>>,
    mut dice_material_assets: ResMut<DiceMaterialAssets>,
    mut dice_mesh_assets: ResMut<DiceMeshAssets>,
    mut text_query: Query<&mut Text>,
    mut r_game_config: ResMut<GameConfig>,
    q_dices: Query<&Dice>
) {
    for (btn, interaction, mut bg, children) in & mut btn_query {
        match btn.button_type {
            GameButtonType::Add => {
                if q_dices.iter().count() < 50 {
                    match *interaction {
                        Interaction::Pressed => {
                            bg.0 = Color::YELLOW;
                            gen_dice(&mut commands, &mut dice_material_assets, &mut dice_mesh_assets)
                        }
                        _ => {
                            bg.0 = Color::WHITE;
                        }
                    }
                } else {
                    bg.0 = Color::WHITE;
                }
            }
            GameButtonType::Remove => {
                match *interaction {
                    Interaction::Pressed => {
                        bg.0 = Color::RED;
                        for dice in despawn_dices.iter() {
                            commands.entity(dice).despawn_recursive();
                            break;
                        }
                    }
                    _ => {
                        bg.0 = Color::WHITE;
                    }
                }
            }
            GameButtonType::Roll => {
                match *interaction {
                    Interaction::Pressed => {
                        bg.0 = Color::BLUE;
                        for ext_impulse in ext_impulses.iter_mut() {
                            impulse_dice(ext_impulse);
                        }
                    }
                    _ => {
                        bg.0 = Color::WHITE;
                    }
                }
            }
            GameButtonType::Mute => {
                let mut text = text_query.get_mut(children[0]).unwrap();
                if *interaction == Interaction::Pressed {
                    if r_game_config.mute {
                        r_game_config.mute = false;
                        text.sections[1].value = "X".to_string();
                    } else {
                        r_game_config.mute = true;
                        text.sections[1].value = "O".to_string();
                    }
                }
            }
            GameButtonType::Motion => {
                let mut text = text_query.get_mut(children[0]).unwrap();
                if *interaction == Interaction::Pressed {
                    if r_game_config.motion {
                        r_game_config.motion = false;
                        text.sections[1].value = "X".to_string();
                    } else {
                        r_game_config.motion = true;
                        text.sections[1].value = "O".to_string();
                    }
                }
            }
        }
    }
}

