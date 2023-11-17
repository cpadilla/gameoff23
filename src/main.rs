use bevy::prelude::*;
use plugin::*;

mod plugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // Insert as resource the initial value for the settings resources
        .insert_resource(ClearColor(BLACK))
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        // Declare the game state, whose starting value is determined by the `Default` trait
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        // Adds the plugins for each state
        .add_plugins((splash::SplashPlugin, menu::MenuPlugin, game::GamePlugin, dialogue::DialoguePlugin, minigame::MinigamePlugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

