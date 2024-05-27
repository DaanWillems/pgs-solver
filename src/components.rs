use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CircleCollider {
    pub radius: f32,
}

#[derive(Component)]
pub struct AABBCollider {
    pub half_size: Vec2,
}

#[derive(Clone, Component)]
pub struct ConvexCollider {
    pub points: Vec<Vec2>,
}

#[derive(Component)]
pub struct Awake(pub bool);

#[derive(Component)]
pub struct Dot;

impl ConvexCollider {
    pub fn from_rect(half_size: Vec2) -> ConvexCollider {
        let top_left = Vec2::new(-half_size.x, half_size.y);
        let top_right = Vec2::new(half_size.x, half_size.y);
        let bottom_right = Vec2::new(half_size.x, -half_size.y);
        let bottom_left = Vec2::new(-half_size.x, -half_size.y);

        let mut points = Vec::new();

        points.push(top_left);
        points.push(top_right);
        points.push(bottom_right);
        points.push(bottom_left);

        return ConvexCollider { points };
    }

    pub fn transform_points(points: &Vec<Vec2>, position: Vec2, rotation: f32) -> Vec<Vec2> {
        let s = f32::sin(rotation);
        let c = f32::cos(rotation);
        let mut new_points = Vec::new();
        for p in points.iter() {
            let new_point = Vec2::new(p.x * c - p.y * s, p.x * s + p.y * c);
            new_points.push(new_point + position);
        }
        return new_points;
    }
}

#[derive(Component)]
pub struct ZLevel(pub f32);

#[derive(Component)]
pub struct Mass(pub f32);

#[derive(Component)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Rotation(pub f32);

#[derive(Component)]
pub struct AngularVelocity(pub f32);

#[derive(Component)]
pub struct Inertia(pub f32);

#[derive(Component)]
pub struct Player;
