use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use crate::utils::*;

use super::components::*;
use super::resources::*;
use super::*;

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    for _ in 0..NUMBER_OF_STARS {
        let x = random::<f32>() * (window.width() - 2.0 * STAR_SIZE) + STAR_SIZE;
        let y = random::<f32>() * (window.height() - 2.0 * STAR_SIZE) + STAR_SIZE;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load(path_join(vec!["sprites", "star.png"])),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn despawn_stars(mut commands: Commands, query: Query<Entity, With<Star>>) {
    for star_entity in query.iter() {
        commands.entity(star_entity).despawn();
    }
}

pub fn tick_star_spawn_timer(mut stat_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    stat_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    let window = window_query.get_single().unwrap();
    if star_spawn_timer.timer.finished() {
        let x = random::<f32>() * (window.width() - 2.0 * STAR_SIZE) + STAR_SIZE;
        let y = random::<f32>() * (window.height() - 2.0 * STAR_SIZE) + STAR_SIZE;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load(path_join(vec!["sprites", "star.png"])),
                ..default()
            },
            Star {},
        ));
    }
}
