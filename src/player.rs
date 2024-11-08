use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use crate::menu::main_menu::MenuState;
use crate::config::GameConfig;

// Player movement system
pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,  // Correction du type pour utiliser Res<Input<KeyCode>> pour capturer les touches
    menu_state: Res<MenuState>,
    config: Res<GameConfig>,
    mut query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    if menu_state.is_open {
        println!("Opening menu - player movement disabled");
        return;
    }

    const SPEED: f32 = 1.0;

    for mut transform in &mut query {
        let mut direction = Vec3::ZERO;

        // Check if the key is pressed and move the player accordingly
        if let Some(forward_key) = config.get_key_forward() {
            if keyboard_input.pressed(forward_key) {
                println!("Move forward with key: {:?}", forward_key);
                direction.z -= 1.0;
            }
        }
        if let Some(backward_key) = config.get_key_backward() {
            if keyboard_input.pressed(backward_key) {
                println!("Move backward with key: {:?}", backward_key);
                direction.z += 1.0;
            }
        }
        if let Some(left_key) = config.get_key_left() {
            if keyboard_input.pressed(left_key) {
                println!("Move left with key: {:?}", left_key);
                direction.x -= 1.0;
            }
        }
        if let Some(right_key) = config.get_key_right() {
            if keyboard_input.pressed(right_key) {
                println!("Move right with key: {:?}", right_key);
                direction.x += 1.0;
            }
        }

        // Display a message if no movement key is detected - can be spammy
        if direction == Vec3::ZERO {
            print!("No movement key detected.");
        }

        let forward = transform.forward();
        let right = transform.right();
        transform.translation += (forward * direction.z + right * direction.x) * SPEED * time.delta_seconds();
    }
}

// Rotate the camera with the mouse
pub fn camera_rotation(
    mut motion_evr: EventReader<MouseMotion>,
    menu_state: Res<MenuState>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    if menu_state.is_open {
        print!("Menu open - camera rotation disabled");
        return;
    }

    const SENSITIVITY: f32 = 0.01;

    for mut transform in &mut query.iter_mut() {
        for ev in motion_evr.read() {
            print!("Camera rotation: delta_x: {}, delta_y: {}", ev.delta.x, ev.delta.y);
            transform.rotate_y(-ev.delta.x * SENSITIVITY);
            transform.rotate_local_x(-ev.delta.y * SENSITIVITY);
        }
    }
}
