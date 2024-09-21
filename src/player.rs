use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Component)]
pub struct Player;

impl Player {
    pub fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(50., 100.))),
                material: materials.add(Color::srgb(0.3, 0.7, 0.5)),
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            })
            .insert(Player);
    }

    pub fn move_player(
        mut query: Query<(&Player, &mut Transform)>,
        input: Res<ButtonInput<KeyCode>>,
    ) {
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
}

impl Plugin for Player {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Player::setup)
            .add_systems(Update, Player::move_player);
    }
}
