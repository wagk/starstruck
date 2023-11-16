mod ship;

use ship::{ship_controller, PlayerShip};

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

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
            .selected_text("Level Selection".to_string())
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

fn spawn_player_assets(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // spawn a camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 6., 12.).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    });

    // spawn a pill
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Cube::default().into()),
            material: materials.add(Color::AQUAMARINE.into()),
            ..default()
        },
        PlayerShip,
        Collider::cuboid(1., 1., 1.),
    ));
}

fn spawn_asteroids(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    const NUM_ASTEROIDS: usize = 5;

    let asteroid = meshes.add(shape::Cube::default().into());

    for _ in 0..NUM_ASTEROIDS {
        let [i, j, k] = rand::random::<[f32; 3]>();
        commands.spawn(PbrBundle {
            mesh: asteroid.clone(),
            transform: Transform::from_translation(
                // Update center
                Vec3::from(rand::random::<[f32; 3]>()) * 5. - 2.5,
            )
            .with_rotation(Quat::from_euler(EulerRot::XYZ, i, j, k))
            .with_scale(Vec3::new(0.5, 0.5, 0.5)),
            ..default()
        });
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_state::<Level>()
        .add_systems(Startup, spawn_player_assets)
        .add_systems(Startup, spawn_asteroids)
        .add_systems(Update, ui_level_selector)
        .add_systems(Update, ship_controller)
        .run();
}
