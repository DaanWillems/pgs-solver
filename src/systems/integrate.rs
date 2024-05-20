use bevy::prelude::*;
use crate::components::*;

pub const DELTA_TIME: f32 = 1. / 60.;

pub fn integrate(mut query: Query<(&mut Velocity, &mut Position, &mut Rotation, &Mass)>) {
    for(mut velocity, mut position, mut rotation, mass) in query.iter_mut() {
        //Apply acceleration forces

        if velocity.0.length() < 0.01 {
            velocity.0.x = 0.;
            velocity.0.y = 0.;
        }

        if mass.0 != 0. {
            velocity.0.y -= 9.81;
        }

        position.0 += velocity.0 * DELTA_TIME;
        velocity.0 *= 0.995;

        rotation.rotation += rotation.angular_velocity * DELTA_TIME;
        // velocity.0.y -= 9.81;
    }
}