use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

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

fn spawn_assets(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    // spawn a camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 6., 12.).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    });

    // spawn a pill
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Capsule::default().into()),
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_state::<Level>()
        .add_systems(Startup, spawn_assets)
        .add_systems(Update, ui_level_selector)
        .run();
}
