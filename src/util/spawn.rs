use bevy::prelude::*;

use bevy::sprite::MaterialMesh2dBundle;

use crate::components::*;

pub fn spawn_circle(commands: &mut Commands, 
    meshes: &mut ResMut<Assets<Mesh>>, 
    materials: &mut ResMut<Assets<ColorMaterial>>,
    pos: Vec2,
    vel: Vec2,
    radius: f32,
    mass: f32,
    player: bool) {

        let shape = Mesh::from(Circle::new(radius));
        let color = ColorMaterial::from(Color::rgb(1., 1., 1.));

        let mesh_handle = meshes.add(shape);
        let material_handle = materials.add(color);
        if player {
            commands.spawn((
                Player,
                Position(pos),
                Velocity(vel),
                Mass(mass),
                Rotation{
                    rotation: 0.,
                    angular_velocity: 0.,
                },
                CircleCollider{radius: radius},
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
                Mass(mass),
                Rotation{
                    rotation: 0.,
                    angular_velocity: 0.,
                },
                CircleCollider{radius: radius},
                MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: material_handle,
                ..default()
                },
            ));
        }
    }

pub fn spawn_rect(commands: &mut Commands, 
    meshes: &mut ResMut<Assets<Mesh>>, 
    materials: &mut ResMut<Assets<ColorMaterial>>,
    pos: Vec2,
    vel: Vec2,
    size: Vec2,
    mass: f32,
    player: bool) {
        let shape = Mesh::from(Rectangle{half_size: size/2.});
        let color = ColorMaterial::from(Color::rgb(1., 1., 1.));

        let mesh_handle = meshes.add(shape);
        let material_handle = materials.add(color);
        if player {
            commands.spawn((
                Player,
                Position(pos),
                Velocity(vel+Vec2::new(0., -0.2)),
                Mass(mass),
                Rotation{
                    rotation: 0.,
                    angular_velocity: 0.,
                },
                AABBCollider{half_size: size/2.},
                MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: material_handle,
                ..default()
                },
            ));
            
        } else {
            commands.spawn((
                Position(pos),
                Velocity(vel+Vec2::new(0., -0.2)),
                Mass(mass),
                Rotation{
                    rotation: 0.,
                    angular_velocity: 0.,
                },
                AABBCollider{half_size: size/2.},
                MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: material_handle,
                ..default()
                },
            ));
        }
    }