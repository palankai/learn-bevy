use super::components::*;
use super::resources::*;
use crate::utils::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use super::*;

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
