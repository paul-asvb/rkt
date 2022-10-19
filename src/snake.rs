use std::f32::MAX;

use crate::actions::Actions;
use crate::{GameState, TIME_STEP};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_prototype_debug_lines::DebugLines;

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

    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::GREEN)),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    });
    // .insert(Snake {
    //     direction: vec3(rng.gen::<f32>(), rng.gen::<f32>(), 0.).normalize(),
    // });
}

fn moving(
    mut lines: ResMut<DebugLines>,
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Snake>>,
) {
    let direction = actions.direction;
    let speed = 100.;

    let start = Vec3::splat(0.0);

    let end = Vec3::new(1000.0, 1000.0, 10.);

    let duration = MAX; // Duration of 0 will show the line for 1 frame.
    lines.line_colored(start, end, duration, Color::PINK);

    let movement = Vec3::new(
        actions.direction.x * speed * TIME_STEP,
        actions.direction.y * speed * TIME_STEP,
        0.,
    );

    // for mut player_transform in &mut player_query {
    //     player_transform.translation += movement;
    // }
}
