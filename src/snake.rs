use std::f32::MAX;
use std::ops::Mul;

use crate::{GameState, TIME_STEP};
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_prototype_debug_lines::DebugLines;
use rand::Rng;

#[derive(Component)]
pub struct Snake {
    direction: Vec3,
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(moving));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();

    commands.spawn_bundle(Camera2dBundle::default());

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::GREEN)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        })
        .insert(Snake {
            direction: vec3(rng.gen::<f32>(), rng.gen::<f32>(), 0.).normalize(),
        });
}

fn moving(
    mut lines: ResMut<DebugLines>,
    time: Res<Time>,
    mut player_query: Query<&mut Snake>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let speed = 100.;

    // let movement = Vec3::new(
    //     actions.direction.x * speed * TIME_STEP,
    //     actions.direction.y * speed * TIME_STEP,
    //     0.,
    // );

    for mut player in &mut player_query {
        //query.direction = query.direction.rotate(ang);

        let old_dir =
            Vec2::new(player.direction.x, player.direction.y).rotate(Vec2::from_angle(1.0));

        if keyboard_input.pressed(KeyCode::Left) {
            //rotation_factor = -rotation_factor;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            //rotation_factor = rotation_factor * -1;
        }
        //,

        player.direction = old_dir.normalize().mul(speed).extend(1.0);

        lines.line_colored(Vec3::splat(0.0), player.direction, MAX, Color::PINK);
    }
}
