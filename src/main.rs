use bevy::prelude::*;

mod game;
mod menu;

#[derive(States, Clone, Eq, PartialEq, Default, Debug, Hash)]
enum AppStates {
    #[default]
    MainMenu, // Display main menu
    Playing, // Active gameplay
    Paused,  // Game paused
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppStates>()
        .add_plugins(game::GamePlugin)
        // .add_plugins(menu::MenuPlugin)
        // .add_systems(Startup, setup)
        // .add_systems(Update, movement_system)
        .run();
}

// fn setup(mut commands: Commands) {
//     // TODO: Spawn things here
// }

// fn movement_system(mut query: Query<&mut Transform>) {
//     // TODO: Movement logic here
// }
