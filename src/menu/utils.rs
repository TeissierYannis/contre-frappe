use bevy::prelude::*;
use super::main_menu::MenuState;

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn spawn_menu_button(parent: &mut ChildBuilder, text: &str, asset_server: &Res<AssetServer>) {
    parent.spawn(ButtonBundle {
        style: Style {
            width: Val::Px(150.0),
            height: Val::Px(50.0),
            margin: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::srgb(0.15, 0.15, 0.15).into(),
        ..default()
    })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
        });
}

pub fn close_menu(menu_state: &mut ResMut<MenuState>, commands: &mut Commands) {
    if let Some(entity) = menu_state.menu_entity.take() {
        commands.entity(entity).despawn_recursive();
    }
    // Reset all menu-related states to default
    menu_state.is_open = false;
    menu_state.in_controls_menu = false;
    menu_state.awaiting_key_change = None;
}


pub fn update_button_text(button_name: &str, new_key: String, text_query: &mut Query<&mut Text>) {
    for mut text in text_query.iter_mut() {
        if text.sections[0].value.contains(button_name) {
            println!("Updating button text for {}: {}", button_name, new_key);
            text.sections[0].value = format!("{} : {}", button_name, new_key);
            break;
        }
    }
}
