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
pub struct Snake;

#[derive(Component)]
pub struct SelfMoving {
    direction: Vec2,
    speed: f32,
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_snake))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_snake));
    }
}

fn spawn_snake(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
        .insert(SelfMoving {
            direction: vec2(rng.gen::<f32>(), rng.gen::<f32>()).normalize(),
            speed: 8.,
        })
        .insert(Snake);
}

fn move_snake(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut SelfMoving, &mut Transform), With<Snake>>,
    //mut player: Query<&mut Snake>,>,
    //mut player: Query<&mut Snake>,
) {
    // let movement = Vec3::new(
    //     actions.direction.x * speed * TIME_STEP,
    //     actions.direction.y * speed * TIME_STEP,
    //     0.,
    // );

    // player position
    let mut q = query.single_mut();

    let direction = q.0.direction;
    let speed = q.0.speed;
    let position = q.1.translation;

    let mut rotation_factor = 0.;
    let mut curviness = 0.1;

    if keyboard_input.pressed(KeyCode::Left) {
        rotation_factor += curviness;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        rotation_factor -= curviness;
    }

    let new_direction = Vec2::new(direction.x, direction.y)
        .rotate(Vec2::from_angle(rotation_factor))
        .normalize();

    let new_player_position = vec2(position.x, position.y) + new_direction * speed;

    q.0.direction = new_direction;
    q.1.translation = new_player_position.extend(1.);
}
