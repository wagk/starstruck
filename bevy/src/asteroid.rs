use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Event)]
pub enum AsteroidUiEvent {
    Shuffle,
    Add(usize),
}

#[derive(Resource)]
pub struct AsteroidMesh(pub Option<Handle<Mesh>>);

#[derive(Component)]
pub struct Asteroid;

fn random_asteroid_transform() -> Transform {
    let [i, j, k] = rand::random::<[f32; 3]>();
    Transform::from_translation(
        // Update center
        Vec3::from(rand::random::<[f32; 3]>()) * 5. - 2.5,
    )
    .with_rotation(Quat::from_euler(EulerRot::XYZ, i, j, k))
    .with_scale(Vec3::new(0.5, 0.5, 0.5))
}

fn make_asteroid(commands: &mut Commands, mesh: &Res<AsteroidMesh>) {
    commands.spawn((
        PbrBundle {
            mesh: mesh.0.clone().unwrap(),
            transform: random_asteroid_transform(),
            ..default()
        },
        Asteroid,
        Collider::cuboid(0.5, 0.5, 0.5),
        ActiveCollisionTypes::STATIC_STATIC,
        ActiveEvents::COLLISION_EVENTS,
    ));
}

pub fn spawn_asteroids(
    mut commands: Commands,
    mut asteroid: ResMut<AsteroidMesh>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    const NUM_ASTEROIDS: usize = 5;

    asteroid.0 = Some(meshes.add(shape::Cube::default().into()));
    let asteroid: Res<AsteroidMesh> = asteroid.into();

    for _ in 0..NUM_ASTEROIDS {
        make_asteroid(&mut commands, &asteroid);
    }
}

pub fn maybe_regenerate_asteroids(
    mut events: EventReader<AsteroidUiEvent>,
    mut commands: Commands,
    mesh: Res<AsteroidMesh>,
    mut asteroids: Query<&mut Transform, With<Asteroid>>,
) {
    for e in events.read() {
        match e {
            AsteroidUiEvent::Shuffle => {
                asteroids.for_each_mut(|mut transform| {
                    *transform = random_asteroid_transform();
                });
            }
            AsteroidUiEvent::Add(n) => {
                for _ in 0..*n {
                    make_asteroid(&mut commands, &mesh)
                }
            }
        }
    }
}
