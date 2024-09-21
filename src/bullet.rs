use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::player::Player;

#[derive(Component)]
pub struct Bullet;

impl Plugin for Bullet {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Bullet::setup)
            .add_systems(Update, Bullet::update);
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
                material: materials.add(Color::srgb(1., 1., 0.)),
                transform: Transform::from_xyz(0., 0., 0.),
                visibility: Visibility::Hidden,
                ..default()
            })
            .insert(Bullet);
    }

    fn update(
        input: Res<ButtonInput<KeyCode>>,
        mut query: Query<(&mut Transform, &mut Visibility, &ViewVisibility), With<Bullet>>,
        player: Query<&Transform, (With<Player>, Without<Bullet>)>,
    ) {
        let player_t = player.single();
        if input.pressed(KeyCode::Space) {
            for (mut transform, mut visibility, _) in query.iter_mut() {
                *visibility = Visibility::Visible;
                transform.translation.x = player_t.translation.x;
                transform.translation.y = player_t.translation.y;
            }
        } else {
            for (mut transform, mut visibility, r_visible) in query.iter_mut() {
                if *visibility == Visibility::Visible && r_visible.get() {
                    transform.translation.y += 10.0;
                }
                if r_visible.get() == false {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}
