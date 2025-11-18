use bevy::app::{App, Plugin, Startup};

pub mod ball;
pub mod paddle;
pub mod scoring;
pub mod systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::setup_camera);
        app.add_systems(Startup, systems::setup_scene);
    }
}
