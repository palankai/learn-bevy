pub mod resources;
mod systems;

use crate::AppState;
use bevy::prelude::*;
use resources::*;
use systems::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(insert_score.in_schedule(OnEnter(AppState::Game)))
            .add_system(remove_score.in_schedule(OnExit(AppState::Game)))
            .init_resource::<HighScore>()
            .add_system(update_score.in_set(OnUpdate(AppState::Game)))
            .add_system(update_high_score)
            .add_system(high_score_updated);
    }
}
