use crate::components::*;
use bevy::prelude::*;

pub const DELTA_TIME: f32 = 1. / 640.;

pub fn integrate(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut Velocity,
        &mut Position,
        &mut Rotation,
        &mut AngularVelocity,
        &Mass,
        &mut Awake
    )>,
) {
    for (entity, mut velocity, mut position, mut rotation, mut angular_velocity, mass, mut awake) in
        query.iter_mut()
    {
        //Apply acceleration forces

        if position.0.y < -1000. {
            commands.entity(entity).despawn();
        }

        if velocity.0.length() > 2. && angular_velocity.0.abs() > 2. {
            awake.0 = true;
        } 

        if velocity.0.length() < 0.1 && angular_velocity.0.abs() < 0.01 {
            velocity.0.x = 0.;
            velocity.0.y = 0.;
            angular_velocity.0 = 0.;
            awake.0 = false;
        }

        if awake.0 && mass.0 != 0. {
            velocity.0.y -= 9.81;
            position.0 += velocity.0 * time.delta_seconds();
            rotation.0 += angular_velocity.0 * time.delta_seconds();
        }


        velocity.0 *= 0.99;
        angular_velocity.0 *= 0.99;

        // velocity.0.y -= 9.81;
    }
}
