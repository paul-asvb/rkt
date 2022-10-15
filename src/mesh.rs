use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct MeshPlugin;

#[derive(Component)]

impl Plugin for MeshPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup));
    }
}

fn create_triangle() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]],
    );
    mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, vec![[0.0, 0.0, 0.0, 1.0]; 3]);
    mesh.set_indices(Some(Indices::U32(vec![0, 1, 2])));
    mesh
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
 
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(MaterialMesh2dBundle {
    mesh: meshes.add(create_triangle()).into(),
    transform: Transform::default().with_scale(Vec3::splat(128.)),
    material: materials.add(ColorMaterial::from(Color::PURPLE)),
    ..Default::default(),
});

}