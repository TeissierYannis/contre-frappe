use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use crate::config::GameConfig;
use crate::menu::utils::{close_menu, spawn_menu_button};

#[derive(Resource, Default)]
pub(crate) struct MenuState {
    pub(crate) is_open: bool,
    pub(crate) in_controls_menu: bool,
    pub(crate) menu_entity: Option<Entity>,
    pub(crate) awaiting_key_change: Option<super::interactions::Direction>,
}

pub fn toggle_menu(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut menu_state: ResMut<MenuState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window>,
    config: Res<GameConfig>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if menu_state.is_open {
            println!("Closing all menus");
            // Ensure all menus are fully closed and reset MenuState when Escape is pressed
            close_menu(&mut menu_state, &mut commands);
            menu_state.is_open = false;
            menu_state.in_controls_menu = false;
            menu_state.awaiting_key_change = None;

            // Lock the cursor when closing the menu completely
            let mut window = windows.single_mut();
            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false;

        } else {
            println!("Opening main menu");
            // Open the main menu and release the cursor
            open_main_menu(&mut commands, &asset_server, &mut menu_state, &config);
            menu_state.is_open = true;

            // Unlock the cursor for menu navigation
            let mut window = windows.single_mut();
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}

pub fn open_main_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    menu_state: &mut ResMut<MenuState>,
    config: &Res<GameConfig>,
) {
    let menu_entity = commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        background_color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(),
        ..default()
    })
        .with_children(|parent| {
            spawn_menu_button(parent, "Retour au jeu", &asset_server);
            spawn_menu_button(parent, "Configurer les contrôles", &asset_server);
            spawn_menu_button(parent, "Quitter le jeu", &asset_server);
        })
        .id();

    menu_state.menu_entity = Some(menu_entity);
}