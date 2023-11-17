use bevy::prelude::*;

use super::{
    GameState,
    LevelState,
    game::Player,
    dialogue::{
        Dialogue,
        DialogueQueue,
        DialogueState
    },
};

#[derive(Resource)]
pub struct Level_0 {
    pub stage: i32
}

#[derive(Resource, Deref, DerefMut)]
pub struct GameTimer(Timer);

// Level 0
pub fn level_0_setup(
    mut dialogue_queue: ResMut<DialogueQueue>,
) {
    // Initial dialogue
    dialogue_queue.queue.push_back(Dialogue{ portrait: 0, text: String::from("...")});
    dialogue_queue.queue.push_back(Dialogue{ portrait: 0, text: String::from("Hey...")});
    dialogue_queue.queue.push_back(Dialogue{ portrait: 0, text: String::from("...")});
    dialogue_queue.queue.push_back(Dialogue{ portrait: 0, text: String::from("You're finally awake.")});
}

pub fn level_0_update(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform)>,
    mut level_state: ResMut<NextState<LevelState>>,
    dialogue_state: Res<State<DialogueState>>,
    mut level0_state: ResMut<Level_0>,
    time: Res<Time>,
    mut timer: Option<ResMut<GameTimer>>,
    mut dialogue_queue: ResMut<DialogueQueue>,
) {

    println!("Stage: {}", level0_state.stage);

    match dialogue_state.get() {
        DialogueState::Disabled => { println!("Disabled");}
        DialogueState::ShowDialogue => {println!("ShowDialogue");}
        DialogueState::Transition => {println!("Transition");}
    }

    match level0_state.stage {
        // Wait for dialogue to display
        0 => {
            match dialogue_state.get() {
                DialogueState::Disabled => {}
                DialogueState::ShowDialogue => {
                    level0_state.stage += 1;
                },
                DialogueState::Transition => {},
            }
        },
        // Wait for dialogue to be finished
        1 => {
            match dialogue_state.get() {
                DialogueState::Disabled => {
                    level0_state.stage += 1;
                }
                DialogueState::ShowDialogue => {},
                DialogueState::Transition => {},
            }
        },
        // Spawn a 1 second timer to trigger the next set of dialogue
        2 => {
            commands.insert_resource(GameTimer(Timer::from_seconds(1.0, TimerMode::Once)));
            level0_state.stage += 1;
        },
        // Wait for timer
        3 => {
            match timer {
                Some(mut timer) => {
                    if timer.tick(time.delta()).finished() {
                        level0_state.stage += 1;
                    }
                },
                None => {}
            }
        },
        // Display more dialogue
        4 => {
            dialogue_queue.queue.push_back(Dialogue{ portrait: 0, text: String::from("Let's bring you in to the station.")});
            dialogue_queue.queue.push_back(Dialogue{ portrait: 0, text: String::from("Use the arrow keys or WASD to bring dock your ship to the station.")});
            level0_state.stage += 1;
        },
        // Wait for dialogue to start
        5 => {
            match dialogue_state.get() {
                DialogueState::Disabled => {}
                DialogueState::ShowDialogue => {
                    level0_state.stage += 1;
                },
                DialogueState::Transition => {},
            }
        },
        // Wait for dialogue to finish
        6 => {
            match dialogue_state.get() {
                DialogueState::Disabled => {
                    level0_state.stage += 1;
                }
                DialogueState::ShowDialogue => {},
                DialogueState::Transition => {},
            }
        },
        // Allow player movement
        // Move this to it's own update function later and just toggle canMove state on player
        7 => {
            let (mut player, mut transform) = query.single_mut();
            
            if keys.just_pressed(KeyCode::W) || keys.just_pressed(KeyCode::Up) {
                transform.translation += Vec3::new(0.0, 10.0, 0.0);
            }
            if keys.just_pressed(KeyCode::S) || keys.just_pressed(KeyCode::Down) {
                transform.translation += Vec3::new(0.0, -10.0, 0.0);
            }
            if keys.just_pressed(KeyCode::A) || keys.just_pressed(KeyCode::Left) {
                transform.translation += Vec3::new(-10.0, 0.0, 0.0);
            }
            if keys.just_pressed(KeyCode::D) || keys.just_pressed(KeyCode::Right) {
                transform.translation += Vec3::new(10.0, 0.0, 0.0);
            }
        },
        // Complete tutorial
        8 => {
            println!("Completed tutorial");
            level_state.set(LevelState::Level_1);
            level0_state.stage += 1;
        },
        i32::MIN..=-1_i32 | 6_i32..=i32::MAX => {}
    }

}