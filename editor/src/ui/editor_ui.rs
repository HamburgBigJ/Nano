use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

pub struct EditorUi;

impl Plugin for EditorUi {
    fn build(&self, app: &mut App) {
        app.add_systems(EguiPrimaryContextPass, test_ui);
    }
}

pub fn test_ui(mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut().expect("No egui context found");

    egui::Window::new("Hello").show(ctx, |ui| {
        ui.label("Hi!");
    });
}