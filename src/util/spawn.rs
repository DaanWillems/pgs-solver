use bevy::prelude::*;

use bevy::sprite::MaterialMesh2dBundle;

use crate::components::*;

pub fn spawn_circle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    pos: Vec2,
    vel: Vec2,
    radius: f32,
    mass: f32,
    player: bool,
) {
    let shape = Mesh::from(Circle::new(radius));
    let color = ColorMaterial::from(Color::rgb(1., 1., 1.));

    let mesh_handle = meshes.add(shape);
    let material_handle = materials.add(color);
    if player {
        commands.spawn((
            Player,
            Position(pos),
            Velocity(vel),
            ZLevel(0.0),
            Mass(mass),
            CircleCollider { radius: radius },
            MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: material_handle,
                ..default()
            },
        ));
    } else {
        commands.spawn((
            Position(pos),
            Velocity(vel),
            ZLevel(0.0),
            Mass(mass),
            CircleCollider { radius: radius },
            MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: material_handle,
                ..default()
            },
        ));
    }
}

pub fn spawn_rect_obb(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    color: Color,
    pos: Vec2,
    vel: Vec2,
    ang_vel: f32,
    rot: f32,
    size: Vec2,
    mass: f32,
    player: bool,
) {
    let shape = Mesh::from(Rectangle {
        half_size: size / 2.,
    });

    let mesh_handle = meshes.add(shape);
    let material_handle = materials.add(color);

    let moment_of_inertia = mass * (size.x * size.x + size.y * size.y) / 12.0;
    if player {
        commands.spawn((
            Player,
            Position(pos),
            ZLevel(0.0),
            Velocity(vel + Vec2::new(0., -0.2)),
            Mass(mass),
            Rotation(rot.to_radians()),
            AngularVelocity(ang_vel),
            Inertia(moment_of_inertia),
            ConvexCollider::from_rect(size / 2.),
            MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: material_handle,
                ..default()
            },
        ));
    } else {
        commands.spawn((
            Position(pos),
            ZLevel(0.0),
            Velocity(vel + Vec2::new(0., -0.2)),
            Mass(mass),
            Rotation(rot.to_radians()),
            AngularVelocity(ang_vel),
            Inertia(moment_of_inertia),
            ConvexCollider::from_rect(size / 2.),
            MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: material_handle,
                ..default()
            },
        ));
    }

}
