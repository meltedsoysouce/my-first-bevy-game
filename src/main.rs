use bevy::{
    prelude::*,
    sprite::{Wireframe2dConfig, Wireframe2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
    color::palettes::*
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Wireframe2dPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands,
         mut meshes: ResMut<Assets<Mesh>>,
         mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());
    let color = Color::srgb(0.3, 0.7, 0.5);

    let shape = Mesh2dHandle(meshes.add(Rectangle::new(50., 100.)));
    commands.spawn(MaterialMesh2dBundle {
        mesh: shape,
        material: materials.add(color),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });
}
