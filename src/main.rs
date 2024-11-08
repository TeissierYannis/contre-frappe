// Importing necessary modules and sub-modules
mod setup; // Contains setup functions for initializing the game environment
mod config; // Holds configuration constants, such as spawn points

// The `systems` module encapsulates functionality for different game systems
mod systems {
    pub mod cursor; // Manages mouse cursor interactions
    pub mod colliders; // Sets up colliders in the scene
    pub mod ui; // Manages UI display, including text elements
    pub mod fps_calculator; // Handles FPS calculations
}

// Importing the Bevy prelude for general game setup and basic systems
use bevy::prelude::*;
// Using Rapier3D for advanced physics and collision handling
use bevy_rapier3d::prelude::*;

// Importing a pre-built FPS controller
use bevy_fps_controller::controller::*;

// Additional imports from local modules
use setup::setup; // Setup functions for initializing environment entities
use systems::{cursor::manage_cursor, colliders::scene_colliders, ui::display_text}; // System functions for managing the cursor, colliders, and UI display
use config::SPAWN_POINT; // SPAWN_POINT constant, used for player or object spawning
use crate::systems::fps_calculator::FpsCalculator; // Custom FPS calculator resource

// Main function that initializes and runs the Bevy app
fn main() {
    App::new()
        // Add FPS calculator as a resource to the app
        .insert_resource(FpsCalculator::new())
        // Setting ambient lighting for the scene
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 10000.0,
        })
        // Setting the clear color of the window background
        .insert_resource(ClearColor(Color::linear_rgb(0.83, 0.96, 0.96)))
        // Adding default plugins provided by Bevy (e.g., for windowing, input handling, etc.)
        .add_plugins(DefaultPlugins)
        // Adding the Rapier physics plugin for 3D physics simulation
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // (Optional) Adding the debug render plugin for visualizing colliders and other physics elements
        // .add_plugins(RapierDebugRenderPlugin::default())
        // Adding the FPS controller plugin for first-person controls
        .add_plugins(FpsControllerPlugin)
        // Adding systems that should run once on startup
        .add_systems(Startup, setup)
        // Adding systems that should run every update frame
        .add_systems(
            Update,
            (manage_cursor, scene_colliders, display_text, respawn),
        )
        // Running the application
        .run();
}

// `respawn` system to reset entities that fall below a certain threshold
fn respawn(mut query: Query<(&mut Transform, &mut Velocity)>) {
    // Iterate over all entities in the query
    for (mut transform, mut velocity) in &mut query {
        // Check if the entity is below a certain Y-level
        if transform.translation.y > -50.0 {
            continue; // Skip if the entity has not fallen
        }

        // Reset the entity's position and velocity
        velocity.linvel = Vec3::ZERO;
        transform.translation = SPAWN_POINT;
    }
}
