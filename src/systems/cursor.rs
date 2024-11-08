use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, MouseButton, Query, Window};
use bevy::window::CursorGrabMode;
use bevy_fps_controller::controller::FpsController;
use crate::Res;

// Function to manage the cursor's behavior based on player input
pub(crate) fn manage_cursor(
    btn: Res<ButtonInput<MouseButton>>,        // Resource to capture mouse button inputs
    key: Res<ButtonInput<KeyCode>>,            // Resource to capture keyboard key inputs
    mut window_query: Query<&mut Window>,      // Query for accessing and modifying the window properties
    mut controller_query: Query<&mut FpsController>, // Query for accessing and controlling the FPS controller
) {
    // Iterate through the window query, allowing adjustments to cursor settings
    for mut window in &mut window_query {

        // Check if the left mouse button was just pressed
        if btn.just_pressed(MouseButton::Left) {
            // Lock the cursor within the window to prevent it from leaving during gameplay
            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false; // Hide the cursor for immersion in FPS mode

            // Enable player input for the FPS controller (movement and actions)
            for mut controller in &mut controller_query {
                controller.enable_input = true;
            }
        }

        // Check if the escape key was just pressed
        if key.just_pressed(KeyCode::Escape) {
            // Release the cursor, allowing it to move freely
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true; // Make the cursor visible for menu interactions

            // Disable player input to the FPS controller
            for mut controller in &mut controller_query {
                controller.enable_input = false;
            }
        }
    }
}
