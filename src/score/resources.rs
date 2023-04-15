use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Resource, Default, Debug)]
pub struct HighScore {
    pub scores: Vec<(String, u32)>,
}
