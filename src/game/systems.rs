use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn setup_scene(mut commands: Commands) {
    commands.spawn(Sprite {
        color: Color::srgb(0.1, 0.1, 0.4),
        custom_size: Some(Vec2::new(800.0, 600.0)),
        ..Default::default()
    });
}
