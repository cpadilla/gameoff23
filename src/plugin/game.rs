use bevy::prelude::*;

use super::{despawn_screen,
    GameState,
    dialogue::{
        DialogueImages,
        DialogueState
    },
    level0::*,
    LevelState
};

// This plugin will contain the game. In this case, it's just be a screen that will
// display the current settings for 5 seconds before returning to the menu
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), game_setup)
            .add_systems(Update, game.run_if(in_state(GameState::Game)))
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>)
            .add_state::<LevelState>()
            .insert_resource(Player {

            })
            // Level 0
            .insert_resource(Level_0 { stage: 0 })
            .add_systems(OnEnter(LevelState::Level_0), level_0_setup)
            .add_systems(Update, level_0_update.run_if(in_state(LevelState::Level_0)));
    }
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

#[derive(Resource)]
struct Player {
    
}

fn game_setup(
    mut dialogue_images: ResMut<DialogueImages>,
    asset_server: Res<AssetServer>,
    mut level_state: ResMut<NextState<LevelState>>
) {

    // Set level 0
    level_state.set(LevelState::Level_0);

    // Load dialogue images
    dialogue_images.map.insert(1, asset_server.load("player.png"));
}

// Main Game Update Loop
fn game(
    time: Res<Time>,
    mut game_state: ResMut<NextState<GameState>>,
    mut level_state: ResMut<NextState<LevelState>>,
    dialogue_state: Res<NextState<DialogueState>>
) {

    // Return to the menu screen after timer finishes
    // if timer.tick(time.delta()).finished() {
        // game_state.set(GameState::Menu);
    // }
}
