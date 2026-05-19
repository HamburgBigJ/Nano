use bevy::app::{Plugin, Update};
use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::prelude::{Commands, MouseButton, Query, Res, ResMut, With};
use bevy::window::{PrimaryWindow, Window};
use crate::elements::elements::ElementRegistry;
use crate::SCALE;
use crate::script::mod_loader::ModsLoaded;
use crate::world::SandWorld;

pub struct GameController;

impl Plugin for GameController {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(SelectedElement { id: 1 });
        app.add_systems(Update, (spawn_element_list, mouse_input_update, button_feedback, button_select));
    }
}



fn mouse_input_update(
    buttons: Res<ButtonInput<MouseButton>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    interaction_q: Query<&Interaction, With<Button>>,
    selected_element: Res<SelectedElement>,
    mut world: ResMut<SandWorld>,
) {

    for interaction in &interaction_q {
        if *interaction == Interaction::Hovered
            || *interaction == Interaction::Pressed
        {
            return;
        }
    }

    let window = window_q.single();

    if buttons.pressed(MouseButton::Left) {
        if let Some(position) = window.unwrap().cursor_position() {
            world.set_cell(position.x as u32 / SCALE as u32, position.y as u32 / SCALE as u32, selected_element.id);
        }
    }
    else if buttons.pressed(MouseButton::Right) {
        if let Some(position) = window.unwrap().cursor_position() {
            world.set_cell(position.x as u32 / SCALE as u32, position.y as u32 / SCALE as u32, 0);
        }
    }
}

#[derive(Resource)]
pub struct SelectedElement {
    pub id: u8,
}

#[derive(Component)]
pub struct ElementButton {
    pub id: u8,
}

#[derive(Component)]
pub struct ElementListUi;

// examples are on the bevy websides
fn spawn_element_list(
    mut commands: Commands,
    elements: Res<ElementRegistry>,
    mods_loaded: Res<ModsLoaded>,
    existing_list: Query<(), With<ElementListUi>>,
) {
    if !mods_loaded.loaded || !existing_list.is_empty() {
        return;
    }

    commands
        .spawn((
            ElementListUi,
            Node {
                width: px(180),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                row_gap: px(6),
                padding: UiRect::all(px(8)),
                ..default()
            },
        ))
        .with_children(|parent| {
            for (id, element) in elements.elements.iter().enumerate() {
                parent
                    .spawn((
                        Button,
                        ElementButton { id: id as u8 },
                        Node {
                            width: percent(100),
                            height: px(36),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    ))
                    .with_children(|button| {
                        button.spawn((
                            Text::new(element.name.clone()),
                            TextColor(Color::WHITE),
                        ));
                    });
            }
        });
}


fn button_feedback(
    mut selected: ResMut<SelectedElement>,
    mut buttons: Query<
        (&Interaction, &ElementButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, element_button, mut color) in &mut buttons {
        match *interaction {
            Interaction::Pressed => {
                selected.id = element_button.id;
                *color = BackgroundColor(Color::srgb(0.8, 0.6, 0.1));
            }

            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
            }

            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.15, 0.15, 0.15));
            }
        }
    }


}

fn button_select(
    selected: ResMut<SelectedElement>,
    mut button_f: Query<
        (&ElementButton, &mut BackgroundColor),
        With<Button>,
    >
) {
    for (button, mut color) in &mut button_f {
        if button.id == selected.id {
            *color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
        } else {
            *color = BackgroundColor(Color::srgb(0.15, 0.15, 0.15));
        }
    }
}
