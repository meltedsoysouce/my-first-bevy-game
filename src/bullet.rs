use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Component)]
pub struct Bullet;

impl Plugin for Bullet {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Bullet::setup);
    }
}

impl Bullet {
    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(10., 10.))),
                material: materials.add(Color::srgb(0.7, 0.3, 0.5)),
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            })
            .insert(Bullet);
    }
}
