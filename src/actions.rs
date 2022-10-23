use bevy::prelude::{debug, App, Input, KeyCode, Plugin, Res, ResMut, SystemSet, Vec2};

use crate::GameState;

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
    let mut rotation_factor: i8 = 10;

    if keyboard_input.pressed(KeyCode::Left) {
        rotation_factor = rotation_factor * 1;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        rotation_factor = rotation_factor * -1;
    }

    let ang = Vec2::from_angle(rotation_factor.into());

    actions.direction = actions.direction.rotate(ang);

    println!("{}", actions.direction)
}
