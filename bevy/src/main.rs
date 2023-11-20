mod asteroid;
mod ship;

use asteroid::{maybe_regenerate_asteroids, spawn_asteroids, AsteroidMesh, AsteroidUiEvent};
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
    mut should_reload: EventWriter<AsteroidUiEvent>,
) {
    egui::Window::new("Debug Menu").show(contexts.ctx_mut(), |ui| {
        // Stage select
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

        // Reload scene
        if ui.button("Shuffle Asteroids").clicked() {
            should_reload.send(AsteroidUiEvent::Shuffle);
        }

        if ui.button("Add Asteroids").clicked() {
            should_reload.send(AsteroidUiEvent::Add(1usize));
        }
    });
}

fn spawn_player_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // spawn a camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 6., 12.).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    });

    // spawn a pill
    let ent = commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Cube::default().into()),
            material: materials.add(Color::AQUAMARINE.into()),
            ..default()
        },
        PlayerShip,
        Collider::cuboid(1., 1., 1.),
        ActiveCollisionTypes::STATIC_STATIC,
        ActiveEvents::COLLISION_EVENTS,
    ));

    println!("Ship has entity ID of {:?}", ent.id());
}

fn display_collision_events(mut collision_events: EventReader<CollisionEvent>) {
    for e in collision_events.read() {
        match e {
            CollisionEvent::Started(e1, e2, _flags) => {
                println!("{e1:?} began collision with {e2:?}");
            }
            CollisionEvent::Stopped(e1, e2, _flags) => {
                println!("{e1:?} ended collision with {e2:?}");
            }
        }
    }
}

fn main() {
    App::new()
        .add_state::<Level>()
        .add_event::<AsteroidUiEvent>()
        .insert_resource(AsteroidMesh(None))
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, spawn_player_assets)
        .add_systems(Startup, spawn_asteroids)
        .add_systems(Update, ui_level_selector)
        .add_systems(Update, ship_controller)
        .add_systems(Update, display_collision_events)
        .add_systems(Last, maybe_regenerate_asteroids)
        .run();
}
