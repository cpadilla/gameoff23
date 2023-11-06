use bevy::{prelude::*};
use itertools::Itertools;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, hover_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::ColumnReverse,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: BackgroundColor(Color::BLACK),
        ..default()
    }).with_children(|parent| {
        parent
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Auto,
                    height: Val::Auto,
                    // border: Rect::all(Val::Px(2.0)),
                    flex_direction: FlexDirection::ColumnReverse,
                    ..default()
                },
                background_color: BackgroundColor(Color::WHITE),
                ..default()
            })
            .with_children(|parent| {
                for row_index in 0..3 {
                    parent.spawn(NodeBundle {
                        style: Style {
                            // size: Size::new(Val::Auto, Val::Auto),

                            width: Val::Auto,
                            height: Val::Auto,
                            ..default()
                        },
                        ..default()
                    }).with_children(|parent| {
                        for column_index in 1..=3 {
                            let cell_id = 3 * row_index + column_index - 1;
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Px(50.0),
                                        height: Val::Px(50.0),
                                        // border: Rect::all(Val::Px(2.0)),
                                        ..default()
                                    },
                                    // color: Color::GREEN,
                                    background_color: BackgroundColor(Color::GREEN),
                                    // background_color: Color::GREEN.into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn(ButtonBundle {
                                            style: Style {
                                                width: Val::Percent(100.0),
                                                height: Val::Percent(100.0),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            background_color: BackgroundColor(Color::YELLOW),
                                            // background_color: Color::YELLOW.into(),
                                            ..default()
                                        });
                                        // .with_children(|parent| {
                                        //     parent.spawn(button_text(
                                        //         &asset_server,
                                        //         &theme,
                                        //         "",
                                        //     ));
                                        // });
                                        // .insert(TicTacToeCell {
                                        //     cell_id,
                                        //     state: CellState::Empty,
                                        // });
                                });
                        }
                    });
                }
            });
    });
}

fn bain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    const TILE_SIZE: f32 = 40.0;

    struct Board {
        size: u8,
    }
    
    let board = Board { size: 4 };
    const TILE_SPACER: f32 = 10.0;

    let physical_board_size = f32::from(board.size)
        * TILE_SIZE
        + f32::from(board.size + 1) * TILE_SPACER;

    let offset = -physical_board_size / 2.0
        + 0.5 * TILE_SIZE;

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(physical_board_size, physical_board_size)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
        ..default()
    }).with_children(|builder| {
        for tile in (0..board.size)
        .cartesian_product(0..board.size)
        {

            builder.spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(TILE_SIZE),
                    height: Val::Px(TILE_SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(
                    offset
                        + f32::from(1.0) * TILE_SIZE
                        + f32::from(1.0)
                            * TILE_SPACER,
                    offset
                        + f32::from(1.1) * TILE_SIZE
                        + f32::from(1.1)
                            * TILE_SPACER,
                    1.0,
                ),
                z_index: ZIndex::Global(999),
                background_color: Color::ORANGE_RED.into(),
                ..default()
            });


            
            // builder.spawn(SpriteBundle {
            //     sprite: Sprite {
            //         custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            //         color: Color::rgb(5.0, 5.0, 5.0),
            //         ..default()
            //     },
            //     transform: Transform::from_xyz(
            //         offset
            //             + f32::from(tile.0) * TILE_SIZE
            //             + f32::from(tile.0 + 1)
            //                 * TILE_SPACER,
            //         offset
            //             + f32::from(tile.1) * TILE_SIZE
            //             + f32::from(tile.1 + 1)
            //                 * TILE_SPACER,
            //         1.0,
            //     ),
            //     ..default()
            // });
            // builder.spawn(ButtonBundle {
            //     style: Style {
            //         width: Val::Px(TILE_SIZE),
            //         height: Val::Px(TILE_SIZE),
            //         // align_items: AlignItems::Center,
            //         // justify_content: JustifyContent::Center,
            //         ..default()
            //     },
            //     transform: Transform::from_xyz(
            //         offset
            //             + f32::from(tile.0) * TILE_SIZE
            //             + f32::from(tile.0 + 1)
            //                 * TILE_SPACER,
            //         offset
            //             + f32::from(tile.1) * TILE_SIZE
            //             + f32::from(tile.1 + 1)
            //                 * TILE_SPACER,
            //         1.0,
            //     ),
            //     z_index: ZIndex::Local(999),
            //     background_color: Color::ORANGE_RED.into(),
            //     ..default()
            // });
        }
    });

    // text
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::FlexEnd,
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section(
                "test",
                TextStyle {
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ),
            ..default()
        });

    });

    // btn
    // TODO: call a function on click
    commands.spawn(ButtonBundle {
        style: Style {
            width: Val::Px(100.0),
            height: Val::Px(30.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::End,
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section(
                "button",
                TextStyle {
                    font_size: 20.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ),
            ..default()
        });
    });
}


fn hover_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
  for (interaction, mut background_color) in interaction_query.iter_mut() {
      match *interaction {
          Interaction::Pressed => {
            println!("pressed")
          },
          Interaction::Hovered => {
            println!("h")
            
          },
          Interaction::None => {
            // println!("n")
            
          },
      }
  }
}