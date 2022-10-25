use std::f32::MAX;
use std::ops::Mul;

use crate::{GameState, TIME_STEP};
use bevy::math::{vec2, vec3};
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::Windows;
use bevy::{prelude::*, window};
use bevy_prototype_debug_lines::DebugLines;
use rand::Rng;

#[derive(Component)]
pub struct Snake {
    direction: Vec2,
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_snake));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<window::Windows>,
) {
    let mut rng = rand::thread_rng();

    commands.spawn_bundle(Camera2dBundle::default());

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::GREEN)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        })
        .insert(Snake {
            direction: vec2(rng.gen::<f32>(), rng.gen::<f32>()).normalize(),
        });
}

fn move_snake(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_transform: Query<&mut Transform>,
    mut player: Query<&mut Snake>,
) {

    // let movement = Vec3::new(
    //     actions.direction.x * speed * TIME_STEP,
    //     actions.direction.y * speed * TIME_STEP,
    //     0.,
    // );

    // player position
    let direction = player.single_mut().direction;



    let mut rotation_factor = 1.;

    if keyboard_input.pressed(KeyCode::Left) {
        rotation_factor = -rotation_factor;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        rotation_factor = rotation_factor;
    }

    let new_snake_direction =
        Vec2::new(direction.x, direction.y).rotate(Vec2::from_angle(rotation_factor));

    player.single_mut().direction = new_snake_direction;

    let position = player_transform.single_mut().translation;
    let new_player_position = vec2(position.x + 1., position.y + 1.);
    println!("{},{}",new_snake_direction,direction);
    player_transform.single_mut().translation =
        (new_player_position + new_snake_direction).extend(1.);
}
