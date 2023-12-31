use bevy::{prelude::*};
use itertools::Itertools;
use super::{despawn_screen, GameState};

pub struct MinigamePlugin;

impl Plugin for MinigamePlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_state::<MinigameState>()
            .add_systems(OnEnter(GameState::Minigame), setup)
            .add_systems(Update, (click, keyboard))
            .add_systems(OnExit(GameState::Minigame), despawn_screen::<MinigameScreen>);
    }
}

// Tag component used to tag entities added on this scene
#[derive(Component)]
struct MinigameScreen;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    const GRID_SIZE: usize = 6;
    const BUTTON_SIZE: f32 = CELL_SIZE / 1.0;
    const LINE_THICKNESS: f32 = 10.0;
    const BUTTON_MARGIN: f32 = (CELL_SIZE + LINE_THICKNESS - BUTTON_SIZE) / 2.0;
    const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
    const FONT_SIZE: f32 = 40.0;
    const LINES_COUNT: usize = GRID_SIZE + 1;
    const CELL_SIZE: f32 = 80.0;
    const BOARD_SIZE: f32 = CELL_SIZE * GRID_SIZE as f32 + LINES_COUNT as f32 * LINE_THICKNESS;
    const BOARD_COLOR: Color = Color::WHITE;

    // commands.spawn(Camera2dBundle::default());

    for line in 0..LINES_COUNT {
      let position = -BOARD_SIZE / 2.0
          + line as f32 * (CELL_SIZE + LINE_THICKNESS)
          + LINE_THICKNESS / 2.0;

      // Horizontal
      commands.spawn((SpriteBundle {
          sprite: Sprite {
              color: BOARD_COLOR,
              ..default()
          },
          transform: Transform {
              translation: Vec3::Y * position,
              scale: Vec3::new(BOARD_SIZE, LINE_THICKNESS, 1.0),
              ..default()
          },
          ..default()
      }, MinigameScreen));

      // Vertical
      commands.spawn((SpriteBundle {
          sprite: Sprite {
              color: BOARD_COLOR,
              ..default()
          },
          transform: Transform {
              translation: Vec3::X * position,
              scale: Vec3::new(LINE_THICKNESS, BOARD_SIZE, 1.0),
              ..default()
          },
          ..default()
      }, MinigameScreen));
  }

    commands.spawn((NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    }, MinigameScreen))
    .with_children(|parent| {
        parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    width: Val::Px(BOARD_SIZE - LINE_THICKNESS),
                    height: Val::Px(BOARD_SIZE - LINE_THICKNESS),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent.spawn((
                        NodeBundle {
                            style: Style {
                                display: Display::Grid,
                                grid_template_columns: vec![GridTrack::auto(); GRID_SIZE],
                                ..default()
                            },
                            ..default()
                        },
                    ))
                    .with_children(|parent| {
                        for _ in 0..GRID_SIZE * GRID_SIZE {
                            parent.spawn(ButtonBundle {
                                style: Style {
                                    width: Val::Px(BUTTON_SIZE),
                                    height: Val::Px(BUTTON_SIZE),
                                    margin: UiRect::all(Val::Px(BUTTON_MARGIN)),
                                    ..default()
                                },
                                ..default()
                            });
                        }
                    });

                parent.spawn(NodeBundle {
                        style: Style {
                            margin: UiRect::top(Val::Px(20.0)),
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_sections([
                                TextSection::new(
                                    String::new(),
                                    TextStyle {
                                        font_size: FONT_SIZE,
                                        color: TEXT_COLOR,
                                        ..default()
                                    },
                                ),
                                TextSection::new(
                                    String::new(),
                                    TextStyle {
                                        font_size: FONT_SIZE,
                                        ..default()
                                    },
                                ),
                            ]),
                        ));
                    });
            });
    });
}


fn click(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
) {
  for (interaction, mut background_color) in interaction_query.iter_mut() {
    *background_color = match *interaction {
      Interaction::Pressed => Color::hsl(211.0, 0.9, 0.48).into(),
      // Interaction::Hovered => Color::hsl(211.0, 0.9, 0.48).into(),
      Interaction::Hovered => Color::WHITE.into(),
      Interaction::None => Color::BLACK.into(),

      // Color::hsl(211.0, 0.9, 0.48)
    }
  }
}


fn keyboard(
  mut commands: Commands,
  keyboard_input: Res<Input<KeyCode>>,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  mut query: Query<(
      &mut TextureAtlasSprite,
      &mut Handle<TextureAtlas>,
      Entity,
  )>,
  mut game_state: ResMut<NextState<GameState>>,
) {
  if keyboard_input.just_pressed(KeyCode::Escape) {
      game_state.set(GameState::Menu);
      for (mut sprite, mut p, mut e) in &mut query {


          // INFO: can despawn sprite
          // commands.entity(e).despawn();

          // INFO: can swap sprite
          // let texture_handle = asset_server.load("blast_2.png");
          // let texture_atlas =
          //     TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 7, 1, None, None);
          // let texture_atlas_handle = texture_atlases.add(texture_atlas);
          // sprite.index = 0;
          // *p = texture_atlas_handle;
      }
  }
}
