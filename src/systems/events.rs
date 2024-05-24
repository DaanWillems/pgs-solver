use bevy::prelude::*;

use bevy::sprite::MaterialMesh2dBundle;

use crate::components::*;

#[derive(Event)]
pub struct SpawnDotEvent {
    pub pos: Vec2,
    pub radius: f32
}