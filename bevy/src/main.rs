use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

#[derive(Debug, PartialEq, Resource)]
enum Stage {
    One,
    Two,
}

fn ui_level_selector(mut selected: ResMut<Stage>, mut contexts: EguiContexts) {
    egui::Window::new("Debug Menu").show(contexts.ctx_mut(), |ui| {
        egui::ComboBox::from_label("combobox")
            .selected_text(format!("Level {selected:?}"))
            .show_ui(ui, |ui| {
                ui.selectable_value(selected.as_mut(), Stage::One, "one");
                ui.selectable_value(selected.as_mut(), Stage::Two, "two");
            });
        ui.end_row();
        ui.label("world");
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .insert_resource(Stage::One)
        .add_systems(Update, ui_level_selector)
        .run();
}
