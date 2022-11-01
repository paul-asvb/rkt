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

#[derive(Component)]
struct SnakeSegment;

#[derive(Default, Deref, DerefMut)]
struct SnakeSegments(Vec<Entity>);

pub struct SnakePlugin;

#[derive(Component)]
struct Collider;

#[derive(Default)]
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
                    .with_system(paint_tail)
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
            mesh: meshes.add(shape::Circle::new(5.).into()).into(),
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

fn paint_tail(
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
            mesh: meshes.add(shape::Circle::new(5.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::GRAY)),
            transform: Transform::from_translation(position.extend(1.)),
            ..default()
        })
        .insert(SnakeSegment);
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
    let curviness = 0.1;

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
    mut ball_query: Query<&Transform, With<SnakeHead>>,
    collider_query: Query<(Entity, &Transform), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let ball_transform = ball_query.single_mut();
    let ball_size = Vec2 { x: 5., y: 5. };

    // check collision with walls
    for (collider_entity, transform) in &collider_query {
        let collision = collide(
            ball_transform.translation,
            ball_size,
            transform.translation,
            ball_size,
        );
        println!("{:?}", collision);
        if let Some(collision) = collision {
            // Sends a collision event so that other systems can react to the collision
            collision_events.send_default();

            // only reflect if the ball's velocity is going in the opposite direction of the
            // collision
            match collision {
                _ => {
                    println!("col")
                }
            }
        }
    }
}
