use crate::events::*;

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResized};

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

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}
