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
        .run();
}
