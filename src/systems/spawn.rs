
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::utils::petgraph::dot;

use crate::{Position, Rotation, SpawnDotEvent, ZLevel};
use crate::components::Dot;

pub fn spawn_dot(
    mut ev_spawn_dot: EventReader<SpawnDotEvent>,
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>,
    dots: Query<Entity, With<Dot>>) {

    for (dot_entity) in dots.iter() {
        commands.entity(dot_entity).despawn();
    }

    for ev in ev_spawn_dot.read() {
        let shape = Mesh::from(Circle::new(ev.radius));
        let color = ColorMaterial::from(Color::rgb(1., 0., 0.));
    
        let mesh_handle = meshes.add(shape);
        let material_handle = materials.add(color);
        commands.spawn((
            Dot,
            ZLevel(1.0),
            Position(ev.pos),
            Rotation{
                rotation: 0.,
                angular_velocity: 0.,
            },
            MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: material_handle,
            ..default()
            },
        ));
    }
}