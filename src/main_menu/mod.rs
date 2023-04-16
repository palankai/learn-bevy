use bevy::prelude::*;

mod systems;
use systems::*;

use crate::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_key_q.in_set(OnUpdate(AppState::MainMenu)));
    }
}
