use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use rand::{seq::IteratorRandom, thread_rng};

const NUM_INITIAL_ASTEROIDS: usize = 5;

#[derive(Event)]
pub enum AsteroidUiEvent {
    Shuffle,
    Add(usize),
    Remove(usize),
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
    asteroid.0 = Some(meshes.add(shape::Cube::default().into()));
    let asteroid: Res<AsteroidMesh> = asteroid.into();

    for _ in 0..NUM_INITIAL_ASTEROIDS {
        make_asteroid(&mut commands, &asteroid);
    }
}

#[allow(clippy::type_complexity)] // `ParamSet`
pub fn maybe_regenerate_asteroids(
    mut events: EventReader<AsteroidUiEvent>,
    mut commands: Commands,
    mesh: Res<AsteroidMesh>,
    mut asteroids: ParamSet<(
        Query<&mut Transform, With<Asteroid>>,
        Query<EntityRef, With<Asteroid>>,
    )>,
) {
    for e in events.read() {
        match e {
            AsteroidUiEvent::Shuffle => {
                asteroids
                    .p0()
                    .for_each_mut(|mut t| *t = random_asteroid_transform());
            }
            AsteroidUiEvent::Add(n) => {
                for _ in 0..*n {
                    make_asteroid(&mut commands, &mesh)
                }
            }
            AsteroidUiEvent::Remove(n) => {
                let mut rng = thread_rng();
                asteroids
                    .p1()
                    .into_iter()
                    .choose_multiple(&mut rng, *n)
                    .into_iter()
                    .for_each(|ent| commands.entity(ent.id()).despawn());
            }
        }
    }
}
