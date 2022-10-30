use crate::GameState;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::time::FixedTimestep;
use rand::Rng;

#[derive(Component)]
pub struct Snake;

#[derive(Component)]
pub struct SelfMoving {
    direction: Vec2,
    speed: f32,
}

const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);

#[derive(Component)]
struct SnakeSegment;

#[derive(Default, Deref, DerefMut)]
struct SnakeSegments(Vec<Entity>);

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_snake))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(move_snake)
                    //.with_run_criteria(FixedTimestep::step(1.0)),
            );
    }
}

fn spawn_snake(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let start = Vec3::new(0., 0., 0.);
    commands.spawn_bundle(Camera2dBundle::default());


    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::GREEN)),
            transform: Transform::from_translation(start),
            ..default()
        })
        .insert(SelfMoving {
            direction: vec2(rng.gen::<f32>(), rng.gen::<f32>()).normalize(),
            speed: 4.,
        })
        .insert(Snake);

}

fn spawn_segment(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    position: Vec2,
) -> Entity {
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::YELLOW)),
            transform: Transform::from_translation(position.extend(1.0)),
            ..default()
        })
        .insert(SnakeSegment)
        .id()
}

fn move_snake(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut SelfMoving, &mut Transform), With<Snake>>,
    //mut player: Query<&mut Snake>,>,
    //mut player: Query<&mut Snake>,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
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

    commands
    .spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(5.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::GRAY)),
        transform: Transform::from_translation(position),
        ..default()
    })
    .insert(SnakeSegment);

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

    let new_player_position = vec2(position.x, position.y) + new_direction * speed;



    q.0.direction = new_direction;
    q.1.translation = new_player_position.extend(1.);
}
