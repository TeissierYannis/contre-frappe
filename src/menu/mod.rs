use bevy::prelude::*;
pub(crate) mod main_menu;
mod controls_menu;
mod interactions;
pub(crate) mod capture_key_change;
mod utils;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<main_menu::MenuState>()
            .add_systems(Update, main_menu::toggle_menu)
            .add_systems(Update, interactions::button_interactions)
            //.add_systems(Update, controls_menu::update_slider)
            .add_systems(Update, capture_key_change::capture_key_change);
    }
}
