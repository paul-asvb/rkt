use crate::GameState;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy::sprite::MaterialMesh2dBundle;
use bevy::time::FixedTimestep;
use rand::Rng;

#[derive(Component)]
pub struct SnakeHead;

#[derive(Component)]
pub struct SelfMoving {
    direction: Vec2,
    speed: f32,
}

//const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
const TIMESTEP_PER_SECOND: f32 = 1. / 60.;
const STARTING_SPEED: f32 = 1.;
const SNAKE_SIZE: f32 = 5.;

#[derive(Component)]
struct SnakeSegment;

#[derive(Default, Deref, DerefMut)]
struct SnakeSegments(Vec<Entity>);

pub struct SnakePlugin;

#[derive(Default, Debug)]
struct CollisionEvent;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>();
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_snake))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(check_for_collisions)
                    .with_system(move_snake.before(check_for_collisions))
                    .with_system(move_snake)
                    .with_system(paint_tail.after(check_for_collisions))
                    .with_system(spawn_tail)
                    .with_run_criteria(FixedTimestep::step(TIMESTEP_PER_SECOND as f64)),
            );
    }
}

fn spawn_snake(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let start = Vec3::new(0., 0., 1.);
    commands.spawn_bundle(Camera2dBundle::default());

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(SNAKE_SIZE).into()).into(),
            material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
            transform: Transform::from_translation(start),
            ..default()
        })
        .insert(SelfMoving {
            direction: vec2(rng.gen::<f32>(), rng.gen::<f32>()).normalize(),
            speed: STARTING_SPEED,
        })
        .insert(SnakeHead);
}

fn spawn_tail(
    query: Query<(&SelfMoving, &Transform), With<SnakeHead>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let position = vec2(
        query.single().1.translation.x,
        query.single().1.translation.y,
    );

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(SNAKE_SIZE).into()).into(),
            material: materials.add(ColorMaterial::from(Color::GRAY)),
            transform: Transform::from_translation(position.extend(1.)),
            ..default()
        })
        .insert(SnakeSegment);
}

fn paint_tail(mut col: EventReader<CollisionEvent>, mut commands: Commands) {
    if col.iter().next().is_some() {
        commands.insert_resource(ClearColor(Color::rgb(255., 0., 0.)));
    } else {
        commands.insert_resource(ClearColor(Color::rgb(0., 0., 0.)));
    }
}

fn move_snake(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut SelfMoving, &mut Transform), With<SnakeHead>>,
) {
    let mut q = query.single_mut();

    let direction = q.0.direction;
    let speed = q.0.speed;
    let position = vec2(q.1.translation.x, q.1.translation.y);

    let mut rotation_factor = 0.;
    let curviness = 0.05;

    if keyboard_input.pressed(KeyCode::Left) {
        rotation_factor += curviness;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        rotation_factor -= curviness;
    }

    let new_direction = Vec2::new(direction.x, direction.y)
        .rotate(Vec2::from_angle(rotation_factor))
        .normalize();

    let new_player_position = position + new_direction * speed;

    q.0.direction = new_direction;
    q.1.translation = new_player_position.extend(2.);
}

fn check_for_collisions(
    head_query: Query<&Transform, With<SnakeHead>>,
    tail_query: Query<&Transform, With<SnakeSegment>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let head_transform = head_query.single();

    const SNAKE_SIZE_VEC: Vec2 = Vec2::new(SNAKE_SIZE, SNAKE_SIZE);

    let l = tail_query.iter().len();

    let taken = if l > 20 { l - 20 } else { 0 };

    for (i, tail_transform) in tail_query.iter().take(taken).enumerate() {
        let collision = collide(
            tail_transform.translation,
            SNAKE_SIZE_VEC,
            head_transform.translation,
            SNAKE_SIZE_VEC,
        );
        if let Some(_) = collision {
            collision_events.send(CollisionEvent);
        }
    }
}
