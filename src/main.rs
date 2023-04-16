mod game;
mod main_menu;

use bevy::prelude::*;

mod events;
mod systems;
mod utils;

use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugin(MainPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(MainMenuPlugin)
        .add_system(transition_to_game_state)
        .add_system(transition_to_main_menu_state)
        .run();
}

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_window)
            .add_startup_system(spawn_camera)
            .add_system(update_camera_when_window_resized)
            .add_system(handle_game_over);
    }
}

#[derive(States, Default, Debug, Hash, Clone, Copy, Eq, PartialEq)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
