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
        .insert_resource(Gravity(0.05))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (apply_gravity, apply_velocity).chain())
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
        Velocity(Vec2::new(1.0, 5.0)),
    ));

    commands.spawn((
        Transform::from_xyz(-150.0, -100.0, 0.0),
        Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(50.0, 150.0)),
            ..Default::default()
        },
        Velocity(Vec2::new(0.5, 5.0))
    ));
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.0.extend(0.0);
    }
}

fn apply_gravity(
    gravity: Res<Gravity>,
    mut query: Query<&mut Velocity>,
) {
    for mut velocity in &mut query {
        velocity.0.y -= gravity.0;
    }
}

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Resource)]
struct Gravity(f32);