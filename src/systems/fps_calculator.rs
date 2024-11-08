// fps_calculator.rs

// Import necessary modules for time and resource handling
use std::time::{Duration, Instant};
use bevy::prelude::Resource;

// Define a struct to calculate and track the frames per second (FPS)
#[derive(Resource)]
pub struct FpsCalculator {
    last_update: Instant,  // Timestamp of the last FPS calculation
    frame_count: u32,      // Counter for frames since last FPS update
    fps: f32,              // Holds the calculated FPS
}

impl FpsCalculator {
    // Creates a new FpsCalculator instance with initial values
    pub fn new() -> Self {
        FpsCalculator {
            last_update: Instant::now(),  // Set the initial time to the current moment
            frame_count: 0,               // Initialize frame counter
            fps: 0.0,                     // Initial FPS value set to zero
        }
    }

    // Called every frame to update the FPS calculation
    pub fn update(&mut self) {
        self.frame_count += 1;  // Increment the frame counter
        let now = Instant::now();  // Get the current time
        let elapsed = now - self.last_update;  // Calculate the time elapsed since the last update

        // Check if a full second has passed since the last FPS update
        if elapsed >= Duration::from_secs(1) {
            // Calculate FPS as frames per second
            self.fps = self.frame_count as f32 / elapsed.as_secs_f32();
            self.frame_count = 0;               // Reset frame counter for the next interval
            self.last_update = now;             // Update the last_update time to the current moment
        }
    }

    // Provides the current FPS value, accessible externally for display purposes
    pub fn get_fps(&self) -> f32 {
        self.fps  // Return the calculated FPS value
    }
}
