use bevy::prelude::*;

use crate::game::paddle::{
    PADDLE_HEIGHT, PADDLE_VERTICAL_OFFSET, PADDLE_WIDTH, PaddleBundle, PlayerControlled,
};

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

pub fn spawn_paddles(mut commands: Commands) {
    // Spawn player paddle (bottom)
    commands.spawn((
        PaddleBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, -PADDLE_VERTICAL_OFFSET, 0.0))
                .with_scale(Vec3::new(PADDLE_WIDTH, PADDLE_HEIGHT, 1.0)),
        },
        PlayerControlled,
    ));
    // TODO: Get paddle spawning to work

    // Spawn AI paddle (top)
    commands.spawn((
        PaddleBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, PADDLE_VERTICAL_OFFSET, 0.0))
                .with_scale(Vec3::new(PADDLE_WIDTH, PADDLE_HEIGHT, 1.0)),
        },
        PlayerControlled,
    ));
}
