use bevy::prelude::*;

use super::{despawn_screen, DisplayQuality, GameState, Volume, WHITE, dialogue::{Dialogue, DialogueQueue, DialogueImages}};

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
    mut dialogue_images: ResMut<DialogueImages>,
    asset_server: Res<AssetServer>
) {
    // Initial dialogue
    dialogue_queue.queue.push_back(Dialogue{ portrait: 0, text: String::from("...")});
    dialogue_queue.queue.push_back(Dialogue{ portrait: 0, text: String::from("Hey...")});
    dialogue_queue.queue.push_back(Dialogue{ portrait: 0, text: String::from("...")});
    dialogue_queue.queue.push_back(Dialogue{ portrait: 0, text: String::from("You're finally awake.")});

    // Spawn a 5 seconds timer to trigger going back to the menu
    commands.insert_resource(GameTimer(Timer::from_seconds(30.0, TimerMode::Once)));

    // Load dialogue images
    dialogue_images.map.insert(1, asset_server.load("player.png"));
}

// Tick the timer, and change state when finished
fn game(
    time: Res<Time>,
    mut game_state: ResMut<NextState<GameState>>,
    mut timer: ResMut<GameTimer>,
) {

    // Return to the menu screen after timer finishes
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu);
    }
}