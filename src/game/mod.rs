mod enemy;
mod player;
mod score;
mod star;
mod systems;

use bevy::prelude::*;

use crate::events::GameOver;
use crate::AppState;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

use systems::{pause_simulation, toggle_simulation};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_event::<GameOver>()
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(StarPlugin)
            .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Default, Debug, Hash, Clone, Copy, Eq, PartialEq)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
