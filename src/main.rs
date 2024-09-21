mod player;

use bevy::{
    color::palettes::*,
    ecs::query,
    input::keyboard::Key,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle, Wireframe2dPlugin},
};

use crate::player::Player;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Wireframe2dPlugin))
        .add_plugins(Player)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    let color = Color::srgb(0.3, 0.7, 0.5);

    let shape = Mesh2dHandle(meshes.add(Rectangle::new(50., 100.)));
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        })
        .insert(Player);

    commands.spawn(
        TextBundle::from_section("aaa", TextStyle::default()).with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
}
