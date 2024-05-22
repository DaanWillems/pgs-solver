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
pub struct ConvexCollider {
    pub points: Vec<Vec2>
}

impl ConvexCollider {
    pub fn new(half_size: Vec2) -> ConvexCollider {
        let top_left = Vec2::new(-half_size.x, half_size.y);
        let top_right = Vec2::new(half_size.x, half_size.y);
        let bottom_right = Vec2::new(half_size.x, -half_size.y);
        let bottom_left = Vec2::new(-half_size.x, -half_size.y);

        let mut points = Vec::new();
        points.push(top_left);
        points.push(top_right);
        points.push(bottom_right);
        points.push(bottom_left);

        return ConvexCollider{points};
    }
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