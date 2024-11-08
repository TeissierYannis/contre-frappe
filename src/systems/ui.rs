use bevy::prelude::*; // Import core Bevy components and traits
use bevy_rapier3d::prelude::*; // Import Bevy Rapier for 3D physics
use crate::systems::fps_calculator::FpsCalculator; // Import custom FpsCalculator

// Function to display FPS, velocity, position, and speed of an entity with `Transform` and `Velocity`
pub(crate) fn display_text(
    mut controller_query: Query<(&Transform, &Velocity)>, // Query entities with Transform and Velocity components
    mut text_query: Query<&mut Text>, // Query for mutable access to Text components
    _time: Res<Time>, // Resource providing time-related information, but unused here
    mut fps_calculator: ResMut<FpsCalculator>, // Resource for updating and fetching FPS
) {
    // Update FPS calculation at every frame
    fps_calculator.update();
    let fps = fps_calculator.get_fps(); // Get the current FPS

    // Loop over entities with both Transform and Velocity components
    for (transform, velocity) in &mut controller_query {
        // Loop over Text components to update their displayed values
        for mut text in &mut text_query {
            // Format and set the value of the text to show FPS, velocity, position, and speed
            text.sections[0].value = format!(
                "FPS: {:.1}\nvel: {:.2}, {:.2}, {:.2}\npos: {:.2}, {:.2}, {:.2}\nspd: {:.2}",
                fps,                             // Display current FPS
                velocity.linvel.x,                // X-component of velocity
                velocity.linvel.y,                // Y-component of velocity
                velocity.linvel.z,                // Z-component of velocity
                transform.translation.x,          // X-coordinate of position
                transform.translation.y,          // Y-coordinate of position
                transform.translation.z,          // Z-coordinate of position
                velocity.linvel.xz().length()     // Calculate horizontal speed (XZ plane length)
            );
        }
    }
}
