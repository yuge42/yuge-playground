use bevy::prelude::*;
use bevy::window::WindowPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Yuge's Playground".to_string(),
                resolution: (800, 600).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, move_sprite)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        Transform::from_xyz(-150.0, 100.0, 0.0),
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..Default::default()
        },
    ));

    commands.spawn((
        Transform::from_xyz(-150.0, -100.0, 0.0),
        Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(50.0, 150.0)),
            ..Default::default()
        },
    ));
}

fn move_sprite(mut query: Query<&mut Transform, With<Sprite>>) {
    for mut transform in &mut query {
        transform.translation.x += 1.0;
    }
}