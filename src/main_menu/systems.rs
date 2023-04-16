use bevy::app::AppExit;
use bevy::prelude::*;

//This system handles the Q press and exit the game
pub fn handle_key_q(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if keyboard_input.pressed(KeyCode::Q) {
        app_exit_events.send(AppExit);
    }
}
