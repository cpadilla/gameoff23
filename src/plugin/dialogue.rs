use std::collections::VecDeque;

use bevy::prelude::*;

// This plugin manages the dialogue queue

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DialogueQueue {
                queue: VecDeque::new()
            })
            .add_state::<DialogueState>()
            .add_systems(OnEnter(DialogueState::ShowDialogue), show_dialogue);
    }
}

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum DialogueState {
    ShowDialogue,
    #[default]
    Disabled,
}

// Queue resource for displaying dialogue
#[derive(Resource)]
pub struct DialogueQueue {
    //queue: Queue<Dialogue>
    pub queue: VecDeque<Dialogue>
}

// Each dialogue contains an int for identifying the image to use and the text to display as dialogue
#[derive(Clone)]
pub struct Dialogue {
    pub portrait: i32,
    pub text: String
}

fn show_dialogue(mut dialogue_state: ResMut<NextState<DialogueState>>, mut dialogue_queue: ResMut<DialogueQueue>) {
    while dialogue_queue.queue.len() > 0 {
        let x = dialogue_queue.queue.pop_front();
        match x {
            Some(x) => {
                println!("{} - {}", x.portrait, x.text);
            },
            None => dialogue_state.set(DialogueState::Disabled),
        }
    }
    dialogue_state.set(DialogueState::Disabled);
}