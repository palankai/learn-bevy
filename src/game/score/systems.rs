use super::resources::*;
use crate::events::*;
use bevy::prelude::*;

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    }
}

pub fn update_high_score(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_score: ResMut<HighScore>,
) {
    for event in game_over_event_reader.iter() {
        high_score.scores.push(("Player".to_string(), event.score));
    }
}

pub fn high_score_updated(high_score: Res<HighScore>) {
    if high_score.is_changed() {
        println!("High Scores: {:?}", high_score);
    }
}
