use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerShip;

pub fn ship_controller(
    kb_input: Res<ButtonInput<KeyCode>>,
    // TODO (pangt): mouse input here, once we get the asteroids up
    mut ship: Query<&mut Transform, With<PlayerShip>>,
) {
    let mut transform = ship.single_mut();

    if kb_input.pressed(KeyCode::KeyE) {
        let forward = transform.forward();
        transform.translation += *forward;
    }

    if kb_input.pressed(KeyCode::KeyS) {
        let left = transform.left();
        transform.translation += *left;
    }

    if kb_input.pressed(KeyCode::KeyD) {
        let back = transform.back();
        transform.translation += *back;
    }

    if kb_input.pressed(KeyCode::KeyF) {
        let right = transform.right();
        transform.translation += *right;
    }

    if kb_input.pressed(KeyCode::KeyW) {
        transform.rotate_local(Quat::from_rotation_z(0.1));
    }

    if kb_input.pressed(KeyCode::KeyR) {
        transform.rotate_local(Quat::from_rotation_z(-0.1));
    }
}
