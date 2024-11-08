use bevy::prelude::*;
use crate::config::GameConfig;
use super::main_menu::MenuState;
use super::interactions::Direction;
use super::utils::update_button_text;

pub fn capture_key_change(
    keys: Res<ButtonInput<KeyCode>>,
    mut menu_state: ResMut<MenuState>,
    mut config: ResMut<GameConfig>,
    mut text_query: Query<&mut Text>,
) {
    // Check if a key is pressed and update the control accordingly
    if let Some(direction) = menu_state.awaiting_key_change {
        if let Some(&key) = keys.get_just_pressed().next() {
            println!("Key pressed for change: {:?}", key);

            //  Update the key in the configuration and the button text
            match direction {
                Direction::Forward => {
                    println!("Changing 'Avancer' key to {:?}", key);
                    config.set_key_forward(key);
                    update_button_text("Avancer", format!("{:?}", key), &mut text_query);
                }
                Direction::Backward => {
                    println!("Changing 'Reculer' key to {:?}", key);
                    config.set_key_backward(key);
                    update_button_text("Reculer", format!("{:?}", key), &mut text_query);
                }
                Direction::Left => {
                    println!("Changing 'Gauche' key to {:?}", key);
                    config.set_key_left(key);
                    update_button_text("Gauche", format!("{:?}", key), &mut text_query);
                }
                Direction::Right => {
                    println!("Changing 'Droite' key to {:?}", key);
                    config.set_key_right(key);
                    update_button_text("Droite", format!("{:?}", key), &mut text_query);
                }
            }

            // Reset the key change state
            menu_state.awaiting_key_change = None;
            println!("Key change complete, awaiting_key_change reset");
        }
    }
}
