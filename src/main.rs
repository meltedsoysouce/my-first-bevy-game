mod player;

use bevy::{prelude::*, sprite::Wireframe2dPlugin};

use crate::player::Player;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Wireframe2dPlugin))
        .add_plugins(Player)
        .add_systems(Startup, setup)
        .add_systems(Update, interact)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(
        TextBundle::from_section("aaa", TextStyle::default()).with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
}

fn interact(mut query: Query<&mut Text>, input: Res<ButtonInput<KeyCode>>) {
    let mut txt = query.single_mut();
    if input.just_pressed(KeyCode::Space) {
        txt.sections[0].value = "space key pressed".to_string();
    }
}
