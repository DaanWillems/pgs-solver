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

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}
