use bevy::prelude::*;

use super::{despawn_screen, DisplayQuality, GameState, Volume, WHITE, dialogue::{Dialogue, DialogueQueue, DialogueState}};

// This plugin will contain the game. In this case, it's just be a screen that will
// display the current settings for 5 seconds before returning to the menu
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), game_setup)
            .add_systems(Update, game.run_if(in_state(GameState::Game)))
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
    }
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

fn game_setup(
    mut commands: Commands,
    mut dialogue_queue: ResMut<DialogueQueue>,
    display_quality: Res<DisplayQuality>,
    volume: Res<Volume>,
) {
    // Initial dialogue
    dialogue_queue.queue.push_back(Dialogue{ portrait: 0, text: String::from("...")});
    dialogue_queue.queue.push_back(Dialogue{ portrait: 0, text: String::from("Hey...")});
    dialogue_queue.queue.push_back(Dialogue{ portrait: 0, text: String::from("...")});
    dialogue_queue.queue.push_back(Dialogue{ portrait: 0, text: String::from("You're finally awake.")});

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    // center children
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnGameScreen,
        ))
        .with_children(|parent| {
            // First create a `NodeBundle` for centering what we want to display
            parent
                .spawn(NodeBundle {
                    style: Style {
                        // This will display its children in a column, from top to bottom
                        flex_direction: FlexDirection::Column,
                        // `align_items` will align children on the cross axis. Here the main axis is
                        // vertical (column), so the cross axis is horizontal. This will center the
                        // children
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::BLACK.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Display two lines of text, the second one with the current settings
                    parent.spawn(
                        TextBundle::from_section(
                            "Will be back to the menu shortly...",
                            TextStyle {
                                font_size: 80.0,
                                color: WHITE,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );
                    parent.spawn(
                        TextBundle::from_sections([
                            TextSection::new(
                                format!("quality: {:?}", *display_quality),
                                TextStyle {
                                    font_size: 60.0,
                                    color: Color::BLUE,
                                    ..default()
                                },
                            ),
                            TextSection::new(
                                " - ",
                                TextStyle {
                                    font_size: 60.0,
                                    color: WHITE,
                                    ..default()
                                },
                            ),
                            TextSection::new(
                                format!("volume: {:?}", *volume),
                                TextStyle {
                                    font_size: 60.0,
                                    color: Color::GREEN,
                                    ..default()
                                },
                            ),
                        ])
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );
                });
        });
    // Spawn a 5 seconds timer to trigger going back to the menu
    commands.insert_resource(GameTimer(Timer::from_seconds(5.0, TimerMode::Once)));
}

// Tick the timer, and change state when finished
fn game(
    time: Res<Time>,
    dialogue_queue: Res<DialogueQueue>,
    mut dialogue_state: ResMut<NextState<DialogueState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut timer: ResMut<GameTimer>,
) {
    // Show dialogue if there is anything in the queue
    if dialogue_queue.queue.len() > 0 {
        dialogue_state.set(DialogueState::ShowDialogue);
    }

    // Return to the menu screen after timer finishes
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu);
    }
}