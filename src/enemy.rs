use bevy::{
    math::bounding::*,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Component)]
pub struct Enemy;

impl Plugin for Enemy {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Enemy::setup);
    }
}

impl Enemy {
    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Triangle2d::new(
                    (30., 60.).into(),
                    (-30., 60.).into(),
                    (0., 0.).into(),
                ))),
                material: materials.add(Color::srgb(0.7, 0.3, 0.2)),
                transform: Transform::from_xyz(0., 200., 0.),
                ..default()
            })
            .insert(Enemy);
    }
}
