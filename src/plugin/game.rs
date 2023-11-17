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
            // Level 0
            .insert_resource(Level_0 { stage: 0 })
            .add_systems(OnEnter(LevelState::Level_0), level_0_setup)
            .add_systems(Update, level_0_update.run_if(in_state(LevelState::Level_0)));
    }
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

#[derive(Component)]
pub struct Player {
    active_quest: i32,
    location: Transform,
    score: i32,
    canMove: bool,
    idleAnimation: AnimationIndices,
    movingAnimation: AnimationIndices
}

#[derive (Component, Clone, Copy)]
pub struct AnimationIndices {
    first: usize,
    last: usize
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn game_setup(
    mut commands: Commands,
    mut dialogue_images: ResMut<DialogueImages>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut level_state: ResMut<NextState<LevelState>>
) {

    // Set level 0
    level_state.set(LevelState::Level_0);

    // Load dialogue images
    dialogue_images.map.insert(1, asset_server.load("player.png"));

    // Set up player sprite sheet
    let texture_handle = asset_server.load("ship.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 29.0), 3, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    
    // Set up the player
    let player = Player {
        active_quest: 0,
        location: Transform::from_xyz(0.0, 0.0, 0.0),
        score: 0,
        canMove: false,
        idleAnimation: AnimationIndices { first: 0, last: 0 },
        movingAnimation: AnimationIndices { first: 1, last: 2 }
    };

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(player.idleAnimation.first),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
        player.idleAnimation,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        player
    ));

    // Set up station sprite sheet
    let texture_handle = asset_server.load("station.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(50.0, 50.0), 2, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 1 };

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_scale(Vec3::splat(2.0)).with_translation(Vec3::new(0.0, 200.0, 0.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(1.0, TimerMode::Repeating))
    ));
}

// Main Game Update Loop
fn game(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}