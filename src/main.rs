use bevy::{
    color::palettes::*,
    ecs::query,
    input::keyboard::Key,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle, Wireframe2dConfig, Wireframe2dPlugin},
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Wireframe2dPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
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

#[derive(Component)]
struct Player;

fn update(mut query: Query<(&Player, &mut Transform)>, input: Res<ButtonInput<KeyCode>>) {
    for (_, mut transform) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }

        if input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        transform.translation += direction.normalize_or_zero();
    }
}
