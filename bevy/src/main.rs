use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

#[derive(Debug, Clone, Eq, Hash, PartialEq, States, Default)]
enum Level {
    #[default]
    One,
    Two,
}

fn ui_level_selector(
    mut curr_stage: ResMut<State<Level>>,
    mut selected: ResMut<NextState<Level>>,
    mut contexts: EguiContexts,
) {
    egui::Window::new("Debug Menu").show(contexts.ctx_mut(), |ui| {
        let mut stage = curr_stage.as_mut().get().clone();
        egui::ComboBox::from_label("combobox")
            .selected_text(format!("Level Selection"))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut stage, Level::One, "one");
                ui.selectable_value(&mut stage, Level::Two, "two");
            });
        if curr_stage.get() != &stage {
            selected.set(stage);
        }
        ui.end_row();
        ui.label("world");
    });
}

fn entering_stage_1(){
    println!("Entering stage 1");
}

fn exiting_stage_1(){
    println!("Exiting stage 1");
}

fn entering_stage_2(){
    println!("Entering stage 2");
}

fn exiting_stage_2(){
    println!("Exiting stage 2");
}

fn spawn_basics() {
    // todo!()
}

fn stage_1() {
    // todo!()
}

fn stage_2() {
    // todo!()
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_state::<Level>()
        .add_systems(Startup, spawn_basics)
        .add_systems(OnEnter(Level::One), entering_stage_1)
        .add_systems(OnExit(Level::One), exiting_stage_1)
        .add_systems(OnEnter(Level::Two), entering_stage_2)
        .add_systems(OnExit(Level::Two), exiting_stage_2)
        .add_systems(Update, ui_level_selector)
        .add_systems(Update, stage_1.run_if(in_state(Level::One)))
        .add_systems(Update, stage_2.run_if(in_state(Level::Two)))
        .run();
}
