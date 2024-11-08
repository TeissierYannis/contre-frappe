use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use crate::config::GameConfig;
use super::main_menu::{MenuState, open_main_menu};
use super::controls_menu::open_controls_menu;
use super::utils::{close_menu, NORMAL_BUTTON, HOVERED_BUTTON, PRESSED_BUTTON};


#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Forward,
    Backward,
    Left,
    Right,
}

pub fn button_interactions(
    mut interaction_query: Query<(&Interaction, &Children, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text>,
    mut menu_state: ResMut<MenuState>,
    mut app_exit_events: EventWriter<AppExit>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<GameConfig>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>, // Ajout de `windows` ici
) {
    for (interaction, children, mut background_color) in &mut interaction_query {
        let button_text = text_query.get_mut(children[0]).unwrap().sections[0].value.clone();
        match *interaction {
            Interaction::Pressed => {
                match button_text.as_str() {
                    "Retour au jeu" => {
                        println!("Returning to game");
                        close_menu(&mut menu_state, &mut commands);
                        menu_state.is_open = false;

                        // Capture le curseur à la fermeture complète du menu
                        let mut window = windows.single_mut();
                        window.cursor.grab_mode = CursorGrabMode::Locked;
                        window.cursor.visible = false;
                    }
                    "Configurer les contrôles" => {
                        println!("Opening controls configuration");
                        close_menu(&mut menu_state, &mut commands);
                        open_controls_menu(&mut commands, &asset_server, &config);
                        menu_state.in_controls_menu = true;
                    }
                    "Retour" => {
                        println!("Returning to main menu from controls menu");
                        close_menu(&mut menu_state, &mut commands); // Ferme le menu de configuration
                        open_main_menu(&mut commands, &asset_server, &mut menu_state, &config);
                        menu_state.in_controls_menu = false;
                    }
                    "Quitter le jeu" => {
                        println!("Exiting game");
                        app_exit_events.send(AppExit::Success);
                    }
                    "Avancer" => menu_state.awaiting_key_change = Some(Direction::Forward),
                    "Reculer" => menu_state.awaiting_key_change = Some(Direction::Backward),
                    "Gauche" => menu_state.awaiting_key_change = Some(Direction::Left),
                    "Droite" => menu_state.awaiting_key_change = Some(Direction::Right),
                    _ => {}
                }
                *background_color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => *background_color = HOVERED_BUTTON.into(),
            Interaction::None => *background_color = NORMAL_BUTTON.into(),
        }
    }
}
