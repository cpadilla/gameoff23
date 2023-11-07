use std::collections::VecDeque;
use bevy::{prelude::*, utils::HashMap};
use super::despawn_screen;

// This plugin manages the dialogue queue

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DialogueQueue {
                queue: VecDeque::new()
            })
            .insert_resource(DialogueImages {
                map: HashMap::new()
            })
            .add_state::<DialogueState>()
            .add_systems(OnEnter(DialogueState::ShowDialogue), show_dialogue)
            .add_systems(Update, update);
    }
}

// Tag component used to tag entities added on dialogue screen
#[derive(Component)]
struct DialogueScreen;

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum DialogueState {
    ShowDialogue,
    #[default]
    Disabled,
}

// Hashmap for storing dialogue images
// We don't really need this, but I guess we do
// since images sometimes won't spawn without it
#[derive(Resource)]
pub struct DialogueImages {
    pub map: HashMap<i32, Handle<Image>>
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

fn show_dialogue(
    mut commands: Commands,
    mut dialogue_state: ResMut<NextState<DialogueState>>,
    mut dialogue_queue: ResMut<DialogueQueue>,
    asset_server: Res<AssetServer>
) {

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(200.0),
                    position_type: PositionType::Absolute,
                    // center children
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    bottom: Val::Px(0.0),
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
            DialogueScreen,
        )).with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(98.0),
                    height: Val::Percent(90.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border: UiRect { left: (Val::Px(2.0)), right: (Val::Px(2.0)), top: (Val::Px(2.0)), bottom: (Val::Px(2.0)) },
                    ..default()
                },
                background_color: Color::BLACK.into(),
                border_color: Color::WHITE.into(),
                ..default()
            }).with_children(|parent| {
                parent.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(98.0),
                        height: Val::Percent(90.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::FlexStart,
                        justify_content: JustifyContent::FlexStart,
                        ..default()
                    },
                    background_color: Color::BLACK.into(),
                    ..default()
                }).with_children(|parent| {
                    
                    parent.spawn(ImageBundle {
                        style: Style {
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::FlexStart,
                            justify_content: JustifyContent::FlexStart,
                            ..default()
                        },
                        image: UiImage {
                            texture: asset_server.load("player.png"),
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
                            ..default()
                        },
                        ..default()
                    });

                    let dialogue = dialogue_queue.queue.pop_front();
                    parent.spawn(
                        TextBundle::from_section(
                            dialogue.unwrap().text,
                            TextStyle {
                                font_size: 30.0,
                                color: Color::WHITE.into(),
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(10.0)),
                            ..default()
                        }),
                    );
                });
            });
        });
        
    if dialogue_queue.queue.len() <= 0 {
        dialogue_state.set(DialogueState::Disabled);
    }
}

fn update(
    commands: Commands,
    buttons: Res<Input<MouseButton>>,
    dialogue_queue: Res<DialogueQueue>,
    mut dialogue_state: ResMut<NextState<DialogueState>>,
    to_despawn: Query<Entity, With<DialogueScreen>>
) {
    
    // Show dialogue if there is anything in the queue
    if dialogue_queue.queue.len() > 0 {
        dialogue_state.set(DialogueState::ShowDialogue);
    }
    
    // if continued, dismiss current dialogue
    if buttons.just_pressed(MouseButton::Left) {
        dialogue_state.set(DialogueState::Disabled);
        despawn_screen::<DialogueScreen>(to_despawn, commands);
    }
}