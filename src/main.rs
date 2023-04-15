use bevy::prelude::*;

mod events;
mod systems;
mod utils;

mod enemy;
mod player;
mod score;
mod star;

use events::*;
use systems::*;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<GameOver>()
        .add_startup_system(setup_window)
        .add_startup_system(spawn_camera)
        .add_system(update_camera_when_window_resized)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(StarPlugin)
        .add_plugin(ScorePlugin)
        .run();
}
