use crate::components::*;
use bevy::prelude::*;

pub fn project_positions(
    mut positionables: Query<(&mut Transform, &mut Position, &Rotation, &ZLevel)>,
) {
    for (mut transform, position, rotation, z_level) in &mut positionables {
        transform.translation = position.0.extend(z_level.0);
        transform.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), rotation.0);
    }
}

pub fn spawn_camera(mut commands: Commands, mut windows: Query<&mut Window>) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    let mut window = windows.single_mut();
    window.resolution.set(2000., 1000.);
}
