use crate::components::*;
use crate::events::*;
use crate::resources::*;
use crate::utils::*;

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResized};
use rand::prelude::*;

pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 500.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;

pub fn setup_window(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window_query.get_single_mut().unwrap();
    window.resizable = false;
    window.resolution.set(1200.0, 900.0);
    window.title = "Csaba's game".to_string();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load(path_join(vec!["sprites", "ball_blue_large.png"])),
            ..default()
        },
        Player {},
    ));
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    for _ in 0..NUMBER_OF_ENEMIES {
        // Ensure the enemy is well within the window bounds
        let x = random::<f32>() * (window.width() - 2.0 * ENEMY_SIZE) + ENEMY_SIZE;
        let y = random::<f32>() * (window.height() - 2.0 * ENEMY_SIZE) + ENEMY_SIZE;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load(path_join(vec!["sprites", "ball_red_large.png"])),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

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

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;

        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        translation.x = translation.x.clamp(x_min, x_max);
        translation.y = translation.y.clamp(y_min, y_max);

        player_transform.translation = translation;
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed = false;
        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        if direction_changed {
            let sound_effect_1 = asset_server.load(path_join(vec!["audio", "pluck_001.ogg"]));
            let sound_effect_2 = asset_server.load(path_join(vec!["audio", "pluck_002.ogg"]));

            let sound_effect = if random::<f32>() < 0.5 {
                sound_effect_1
            } else {
                sound_effect_2
            };
            audio.play(sound_effect);
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;

    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation;

        translation.x = translation.x.clamp(x_min, x_max);
        translation.y = translation.y.clamp(y_min, y_max);

        transform.translation = translation;
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        // Add the Code.
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                println!("Player hit!");
                let sound_effect =
                    asset_server.load(path_join(vec!["audio", "explosionCrunch_000.ogg"]));
                audio.play(sound_effect);
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver { score: score.value });
            }
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(&Transform, Entity), With<Star>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        // Add the Code.
        for (star_transform, star_entity) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let star_radius = STAR_SIZE / 2.0;
            if distance < player_radius + star_radius {
                score.value += 1;
                let sound_effect =
                    asset_server.load(path_join(vec!["audio", "laserLarge_000.ogg"]));
                audio.play(sound_effect);
                commands.entity(star_entity).despawn();
            }
        }
    }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    }
}

pub fn update_camera_when_window_resized(
    mut events: EventReader<WindowResized>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    let window = window_query.get_single().unwrap();
    let mut camera = camera_query.get_single_mut().unwrap();
    for _ in events.iter() {
        camera.translation =
            Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0).translation;
    }
}

pub fn handle_game_over(mut events: EventReader<GameOver>) {
    for event in events.iter() {
        println!("Game Over! Final Score: {}", event.score);
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

pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
    let window = window_query.get_single().unwrap();
    if enemy_spawn_timer.timer.finished() {
        let x = random::<f32>() * (window.width() - 2.0 * ENEMY_SIZE) + ENEMY_SIZE;
        let y = random::<f32>() * (window.height() - 2.0 * ENEMY_SIZE) + ENEMY_SIZE;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load(path_join(vec!["sprites", "ball_red_large.png"])),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
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
