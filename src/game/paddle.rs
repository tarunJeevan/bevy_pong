use bevy::{ecs::component::Component, sprite::Sprite, transform::components::Transform};

pub const PADDLE_WIDTH: f32 = 20.0; // Paddle thickness
pub const PADDLE_HEIGHT: f32 = 100.0; // Paddle length

pub const PADDLE_VERTICAL_OFFSET: f32 = 250.0; // Distance form the center

pub const PADDLE_SPEED: f32 = 500.0; // Paddle movement speed

#[derive(Component)]
pub struct PlayerControlled;

#[derive(Component)]
pub struct AIControlled;

#[derive(Component)]
pub struct PaddleBundle {
    pub sprite: Sprite,
    pub transform: Transform,
}
