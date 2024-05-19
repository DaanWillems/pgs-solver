use bevy::prelude::*;
use crate::components::*;

pub fn project_positions(
    mut positionables: Query<(&mut Transform, &mut Position, &Rotation)>
) {
    for (mut transform, position, rotation) in &mut positionables {
        transform.translation = position.0.extend(0.0);
        transform.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), rotation.rotation);
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}
