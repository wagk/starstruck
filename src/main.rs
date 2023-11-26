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

#[derive(Component)]
struct FollowerCamera {
    offset: Vec3,
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

        if ui.button("Add Asteroid").clicked() {
            should_reload.send(AsteroidUiEvent::Add(1usize));
        }

        if ui.button("Remove Asteroid").clicked() {
            should_reload.send(AsteroidUiEvent::Remove(1usize));
        }
    });
}

fn spawn_player_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_transform = Transform::default();

    // spawn a pill
    let ship = commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Cube::default().into()),
            material: materials.add(Color::AQUAMARINE.into()),
            transform: player_transform.clone(),
            ..default()
        },
        PlayerShip,
        Collider::cuboid(1., 1., 1.),
        ActiveCollisionTypes::STATIC_STATIC,
        ActiveEvents::COLLISION_EVENTS,
    ));

    println!("Ship has entity ID of {:?}", ship.id());

    // spawn a camera
    let offset = Vec3::new(0., 5., 5.);
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(player_transform.translation + offset.clone())
                .looking_at(player_transform.translation, Vec3::Y),
            ..default()
        },
        FollowerCamera { offset },
    ));
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

fn update_follower_camera(
    ship: Query<&Transform, (With<PlayerShip>, Without<FollowerCamera>)>,
    mut camera: Query<(&FollowerCamera, &mut Transform), With<FollowerCamera>>,
) {
    let (FollowerCamera { offset }, mut cam_transform) = camera.single_mut();
    let ship = ship.single();

    *cam_transform = cam_transform
        .with_translation(ship.translation + *offset)
        .looking_at(ship.translation, Vec3::Y);

    cam_transform.rotate_around(ship.translation, ship.rotation);
}

fn mouse_controller(mut events: EventReader<CursorMoved>) {
    for CursorMoved { position, .. } in events.read() {
        println!("Mouse position: {:?}", position);
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
        .add_systems(Update, update_follower_camera.after(ship_controller))
        .add_systems(Update, display_collision_events)
        .add_systems(Update, mouse_controller)
        .add_systems(Last, maybe_regenerate_asteroids)
        .run();
}