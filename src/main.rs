mod setup;
mod player;
mod fps_overlay;
mod menu;
mod config;

use bevy::prelude::*;
use fps_overlay::init_fps_overlay;
use menu::MenuPlugin;
use config::GameConfig;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(init_fps_overlay())
        .add_plugins(MenuPlugin)
        .insert_resource(GameConfig::new())
        .add_systems(Startup, setup::setup)
        .add_systems(Update, (player::player_movement, player::camera_rotation))
        .run();
}
