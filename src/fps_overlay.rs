use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::prelude::*;

// This function initializes the FPS overlay plugin with a green color and a font size of 20
pub fn init_fps_overlay() -> FpsOverlayPlugin {
    FpsOverlayPlugin {
        config: FpsOverlayConfig {
            text_config: TextStyle {
                font_size: 20.0,
                color: Color::srgb(0.0, 1.0, 0.0),
                font: default(),
            },
        },
    }
}
