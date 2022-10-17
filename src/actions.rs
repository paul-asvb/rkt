use crate::GameState;
use bevy::{prelude::*, math::vec2};

use rand::Rng; // 0.6.5

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(set_movement_actions),
        );
    }
}

// points in the direction the player is moving
#[derive(Default)]
pub struct Actions {
    pub direction: Vec2,
}

fn set_movement_actions(mut actions: ResMut<Actions>, keyboard_input: Res<Input<KeyCode>>) {
    
    let mut rng = rand::thread_rng();

        let mut player_movement = vec2(rng.gen::<f32>(),rng.gen::<f32>());

        actions.direction = player_movement;

        // if GameControl::Up.just_released(&keyboard_input)
        //     || GameControl::Down.just_released(&keyboard_input)
        // {
        //     if GameControl::Up.pressed(&keyboard_input) {
        //         player_movement.y = 1.;
        //     } else if GameControl::Down.pressed(&keyboard_input) {
        //         player_movement.y = -1.;
        //     } else {
        //         player_movement.y = 0.;
        //     }
        // } else if GameControl::Up.just_pressed(&keyboard_input) {
        //     player_movement.y = 1.;
        // } else if GameControl::Down.just_pressed(&keyboard_input) {
        //     player_movement.y = -1.;
        // } else {
        //     player_movement.y = actions.player_movement.unwrap_or(Vec2::ZERO).y;
        // }

        // if GameControl::Right.just_released(&keyboard_input)
        //     || GameControl::Left.just_released(&keyboard_input)
        // {
        //     if GameControl::Right.pressed(&keyboard_input) {
        //         player_movement.x = 1.;
        //     } else if GameControl::Left.pressed(&keyboard_input) {
        //         player_movement.x = -1.;
        //     } else {
        //         player_movement.x = 0.;
        //     }
        // } else if GameControl::Right.just_pressed(&keyboard_input) {
        //     player_movement.x = 1.;
        // } else if GameControl::Left.just_pressed(&keyboard_input) {
        //     player_movement.x = -1.;
        // } else {
        //     player_movement.x = actions.player_movement.unwrap_or(Vec2::ZERO).x;
        // }

        // if player_movement != Vec2::ZERO {
        //     player_movement = player_movement.normalize();
        //     actions.player_movement = Some(player_movement);
        // }
   
}

enum GameControl {
    Left,
    Right,
}

impl GameControl {
    fn just_released(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Left => {
                keyboard_input.just_released(KeyCode::A)
                    || keyboard_input.just_released(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.just_released(KeyCode::D)
                    || keyboard_input.just_released(KeyCode::Right)
            }
        }
    }

    fn pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Left => {
                keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right)
            }
        }
    }

    fn just_pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Left => {
                keyboard_input.just_pressed(KeyCode::A)
                    || keyboard_input.just_pressed(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.just_pressed(KeyCode::D)
                    || keyboard_input.just_pressed(KeyCode::Right)
            }
        }
    }
}
