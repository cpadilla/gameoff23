use bevy::{prelude::*};
use itertools::Itertools;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
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
            builder.spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    offset
                        + f32::from(tile.0) * TILE_SIZE
                        + f32::from(tile.0 + 1)
                            * TILE_SPACER,
                    offset
                        + f32::from(tile.1) * TILE_SIZE
                        + f32::from(tile.1 + 1)
                            * TILE_SPACER,
                    1.0,
                ),
                ..default()
            });
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
