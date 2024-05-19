use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CircleCollider {
    pub radius: f32
}

#[derive(Component)]
pub struct AABBCollider {
    pub half_size: Vec2
}

#[derive(Component)]
pub struct Mass(pub f32);

#[derive(Component)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct Rotation {
    pub rotation: f32, //In radians
    pub angular_velocity: f32
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Player;