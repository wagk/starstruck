use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerShip;

pub fn ship_controller(
    kb_input: Res<Input<KeyCode>>,
    // TODO (pangt): mouse input here, once we get the asteroids up
    mut ship: Query<&mut Transform, With<PlayerShip>>,
) {
    assert_eq!(
        ship.iter().len(),
        1,
        "There should only be one player-controlled ship"
    );

    for mut transform in &mut ship {
        if kb_input.pressed(KeyCode::E) {
            let forward = transform.forward();
            transform.translation += forward;
        }

        if kb_input.pressed(KeyCode::S) {
            let left = transform.left();
            transform.translation += left;
        }

        if kb_input.pressed(KeyCode::D) {
            let back = transform.back();
            transform.translation += back;
        }

        if kb_input.pressed(KeyCode::F) {
            let right = transform.right();
            transform.translation += right;
        }

        if kb_input.pressed(KeyCode::W) {
            transform.rotate_local(Quat::from_rotation_z(0.1));
        }

        if kb_input.pressed(KeyCode::R) {
            transform.rotate_local(Quat::from_rotation_z(-0.1));
        }
    }
}
