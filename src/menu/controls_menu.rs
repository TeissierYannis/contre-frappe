use bevy::prelude::*;
use crate::config::GameConfig;
use crate::menu::main_menu::MenuState;
use super::utils::spawn_menu_button;

pub fn open_controls_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    config: &Res<GameConfig>,
) {
    println!("Opening controls menu");

    let menu_entity = commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        background_color: Color::srgba(0.1, 0.1, 0.1, 0.8).into(),
        ..default()
    })
        .with_children(|parent| {
            for (label, action) in [
                ("Avancer", config.key_forward.clone()),
                ("Reculer", config.key_backward.clone()),
                ("Gauche", config.key_left.clone()),
                ("Droite", config.key_right.clone())
            ] {
                spawn_menu_button(parent, &format!("{} : {}", label, action), &asset_server);
            }
            spawn_menu_button(parent, "Retour", &asset_server);
        })
        .id();

    commands.insert_resource(MenuState { menu_entity: Some(menu_entity), ..default() });
}
