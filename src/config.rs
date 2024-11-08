use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// Game configuration structure
#[derive(Resource, Serialize, Deserialize, Default)]
pub struct GameConfig {
    pub sensitivity: f32,
    pub key_forward: String,
    pub key_backward: String,
    pub key_left: String,
    pub key_right: String,
}

// Game configuration implementation
impl GameConfig {
    pub fn new() -> Self {
        GameConfig {
            sensitivity: 0.01,
            key_forward: "W".to_string(),
            key_backward: "S".to_string(),
            key_left: "A".to_string(),
            key_right: "D".to_string(),
        }
    }

    // Methode to define the sensitivity
    pub fn set_sensitivity(&mut self, sensitivity: f32) {
        self.sensitivity = sensitivity;
    }

    // Methode to define the keys for the movements
    pub fn set_key_forward(&mut self, key: KeyCode) {
        self.key_forward = GameConfig::keycode_to_string(key);
    }

    pub fn set_key_backward(&mut self, key: KeyCode) {
        self.key_backward = GameConfig::keycode_to_string(key);
    }

    pub fn set_key_left(&mut self, key: KeyCode) {
        self.key_left = GameConfig::keycode_to_string(key);
    }

    pub fn set_key_right(&mut self, key: KeyCode) {
        self.key_right = GameConfig::keycode_to_string(key);
    }

    // Convert KeyCode to String for storage
    fn keycode_to_string(key: KeyCode) -> String {
        format!("{:?}", key)
    }

    // Convert String to KeyCode for use in the game
    pub fn string_to_keycode(key: &str) -> Option<KeyCode> {
        let result = match key {
            // Letters (simple and full formats)
            "A" | "KeyA" => Some(KeyCode::KeyA),
            "B" | "KeyB" => Some(KeyCode::KeyB),
            "C" | "KeyC" => Some(KeyCode::KeyC),
            "D" | "KeyD" => Some(KeyCode::KeyD),
            "E" | "KeyE" => Some(KeyCode::KeyE),
            "F" | "KeyF" => Some(KeyCode::KeyF),
            "G" | "KeyG" => Some(KeyCode::KeyG),
            "H" | "KeyH" => Some(KeyCode::KeyH),
            "I" | "KeyI" => Some(KeyCode::KeyI),
            "J" | "KeyJ" => Some(KeyCode::KeyJ),
            "K" | "KeyK" => Some(KeyCode::KeyK),
            "L" | "KeyL" => Some(KeyCode::KeyL),
            "M" | "KeyM" => Some(KeyCode::KeyM),
            "N" | "KeyN" => Some(KeyCode::KeyN),
            "O" | "KeyO" => Some(KeyCode::KeyO),
            "P" | "KeyP" => Some(KeyCode::KeyP),
            "Q" | "KeyQ" => Some(KeyCode::KeyQ),
            "R" | "KeyR" => Some(KeyCode::KeyR),
            "S" | "KeyS" => Some(KeyCode::KeyS),
            "T" | "KeyT" => Some(KeyCode::KeyT),
            "U" | "KeyU" => Some(KeyCode::KeyU),
            "V" | "KeyV" => Some(KeyCode::KeyV),
            "W" | "KeyW" => Some(KeyCode::KeyW),
            "X" | "KeyX" => Some(KeyCode::KeyX),
            "Y" | "KeyY" => Some(KeyCode::KeyY),
            "Z" | "KeyZ" => Some(KeyCode::KeyZ),

            // Numbers (simple and full formats)
            "0" | "Digit0" => Some(KeyCode::Digit0),
            "1" | "Digit1" => Some(KeyCode::Digit1),
            "2" | "Digit2" => Some(KeyCode::Digit2),
            "3" | "Digit3" => Some(KeyCode::Digit3),
            "4" | "Digit4" => Some(KeyCode::Digit4),
            "5" | "Digit5" => Some(KeyCode::Digit5),
            "6" | "Digit6" => Some(KeyCode::Digit6),
            "7" | "Digit7" => Some(KeyCode::Digit7),
            "8" | "Digit8" => Some(KeyCode::Digit8),
            "9" | "Digit9" => Some(KeyCode::Digit9),

            // Special keys (simple and full formats)
            "Enter" => Some(KeyCode::Enter),
            "Escape" => Some(KeyCode::Escape),
            "Space" => Some(KeyCode::Space),
            "Left" | "ArrowLeft" => Some(KeyCode::ArrowLeft),
            "Right" | "ArrowRight" => Some(KeyCode::ArrowRight),
            "Up" | "ArrowUp" => Some(KeyCode::ArrowUp),
            "Down" | "ArrowDown" => Some(KeyCode::ArrowDown),
            "Tab" => Some(KeyCode::Tab),
            "ShiftLeft" => Some(KeyCode::ShiftLeft),
            "ShiftRight" => Some(KeyCode::ShiftRight),
            "ControlRight" => Some(KeyCode::ControlRight),
            "ControlLeft" => Some(KeyCode::ControlLeft),
            "AltLeft" => Some(KeyCode::AltLeft),
            "AltRight" => Some(KeyCode::AltRight),

            _ => None,
        };
        //println!("Converting string '{}' to keycode {:?}", key, result); // Debug
        result
    }




    // Getters for the keys
    pub fn get_key_forward(&self) -> Option<KeyCode> {
        GameConfig::string_to_keycode(&self.key_forward)
    }

    pub fn get_key_backward(&self) -> Option<KeyCode> {
        GameConfig::string_to_keycode(&self.key_backward)
    }

    pub fn get_key_left(&self) -> Option<KeyCode> {
        GameConfig::string_to_keycode(&self.key_left)
    }

    pub fn get_key_right(&self) -> Option<KeyCode> {
        GameConfig::string_to_keycode(&self.key_right)
    }
}
