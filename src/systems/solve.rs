use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

use super::integrate::DELTA_TIME;

pub fn pre_step(
    query: Query<(
        &mut Position,
        &mut Velocity,
        &mut AngularVelocity,
        &Inertia,
        &Mass,
    )>,
    time: Res<Time>,
    mut manifolds: ResMut<Manifolds>,
) {
    for manifold in manifolds.0.iter_mut() {
        let (
            (mut pos_a, mut vel_a, mut ang_vel_a, inertia_a, mass_a),
            (mut pos_b, mut vel_b, mut ang_vel_b, inertia_b, mass_b),
        ) = unsafe {
            assert!(manifold.entity_a != manifold.entity_b);
            (
                query.get_unchecked(manifold.entity_a).unwrap(),
                query.get_unchecked(manifold.entity_b).unwrap(),
            )
        };
        let w_a;
        let w_b;
        let i_a;
        let i_b;

        if mass_a.0 == 0. {
            w_a = 0.;
            i_a = 0.;
        } else {
            w_a = 1. / mass_a.0;
            i_a = 1. / inertia_a.0;
        }

        if mass_b.0 == 0. {
            w_b = 0.;
            i_b = 0.;
        } else {
            w_b = 1. / mass_b.0;
            i_b = 1. / inertia_b.0;
        }
        let meff = 1. / (w_a + w_b);

        let mut impulses = Vec::new();

        for contact_point in manifold.contact_points.clone() {
            let b = -(0.5 / time.delta_seconds()) * (f32::max(0., manifold.overlap - 0.05  ));

            let ra = contact_point - pos_a.0;
            let rb = contact_point - pos_b.0;
            let ra_perp = ra.perp();
            let rb_perp = rb.perp();

    

            let angular_linear_velocity_a = ra_perp * ang_vel_a.0;
            let angular_linear_velocity_b = rb_perp * ang_vel_b.0;

            let ra_perp_n = ra_perp.dot(manifold.normal);
            let rb_perp_n = rb_perp.dot(manifold.normal);

            manifold.set_bias(b);

            let e = 0.0;

            let rel_v =
                (vel_b.0 + angular_linear_velocity_b) - (vel_a.0 + angular_linear_velocity_a);

            let rel_normal_v = rel_v.dot(manifold.normal);

            if rel_normal_v > 0. {
                continue;
            }

            let restitution = e * (rel_normal_v);

            // let denom = w_a + w_b;
            let denom = (w_a + w_b + ((ra_perp_n * ra_perp_n) * i_a) + ((rb_perp_n * rb_perp_n) * i_b));
            let mut j = (-(1. + e) * (rel_normal_v)) - manifold.bias;
            j /= denom;
            j /= manifold.contact_points.len() as f32;
            let impulse = j * manifold.normal;

            impulses.push((ra, rb, impulse));
        }

        for (ra, rb, impulse) in impulses {
            vel_a.0 += -impulse * w_a;
            ang_vel_a.0 += -Vec2::perp_dot(ra, impulse) * i_a;
            vel_b.0 += impulse * w_b;
            ang_vel_b.0 += Vec2::perp_dot(rb, impulse) * i_b;
        }
    }
}

pub fn apply_impulses(
    query: Query<(
        &mut Position,
        &mut Velocity,
        &mut AngularVelocity,
        &Inertia,
        &Mass,
    )>,
    manifolds: Res<Manifolds>,
) {
    for _ in 0..8 {
        for manifold in manifolds.0.iter() {
            let (
                (mut pos_a, mut vel_a, mut ang_vel_a, inertia_a, mass_a),
                (mut pos_b, mut vel_b, mut ang_vel_b, inertia_b, mass_b),
            ) = unsafe {
                assert!(manifold.entity_a != manifold.entity_b);
                (
                    query.get_unchecked(manifold.entity_a).unwrap(),
                    query.get_unchecked(manifold.entity_b).unwrap(),
                )
            };
            let w_a;
            let w_b;
            let i_a;
            let i_b;

            if mass_a.0 == 0. {
                w_a = 0.;
                i_a = 0.;
            } else {
                w_a = 1. / mass_a.0;
                i_a = 1. / inertia_a.0;
            }

            if mass_b.0 == 0. {
                w_b = 0.;
                i_b = 0.;
            } else {
                w_b = 1. / mass_b.0;
                i_b = 1. / inertia_b.0;
            }

            let meff = 1. / (w_a + w_b);
            let mut impulses = Vec::new();

            for contact_point in manifold.contact_points.clone() {
                let ra = contact_point - pos_a.0;
                let rb = contact_point - pos_b.0;
                let ra_perp = ra.perp();
                let rb_perp = rb.perp();

                let angular_linear_velocity_a = ra_perp *  ang_vel_a.0;
                let angular_linear_velocity_b = rb_perp * ang_vel_b.0;

                let ra_perp_n = ra_perp.dot(manifold.normal);
                let rb_perp_n = rb_perp.dot(manifold.normal);

                let e = 0.1;

                let rel_v =
                    (vel_b.0 + angular_linear_velocity_b) - (vel_a.0 + angular_linear_velocity_a);

                let rel_normal_v = rel_v.dot(manifold.normal);
                if rel_normal_v > 0. {
                    continue;
                }
                // let restitution = e * (rel_normal_v);

                // let mut j = ((-(1. + restitution) * (rel_normal_v)) - manifold.bias) / (w_a + w_b);

                let denom = (w_a + w_b + ((ra_perp_n * ra_perp_n) * i_a) + ((rb_perp_n * rb_perp_n) * i_b));
                // let denom = w_a + w_b;
                let mut j = (-(1. + e) * (rel_normal_v)) - manifold.bias;
                j /= denom;

                j /= manifold.contact_points.len() as f32;

                let impulse = j * manifold.normal;

                impulses.push((ra, rb, impulse));

                // vel_a.0 += -j * manifold.normal * w_a;
                // ang_vel_a.0 += -Vec2::perp_dot(ra, j * manifold.normal) * i_a;
                // vel_b.0 += j * manifold.normal * w_b;
                // ang_vel_b.0 += Vec2::perp_dot(rb, j * manifold.normal) * i_b;
            }

            for (ra, rb, impulse) in impulses {
                vel_a.0 += -impulse * w_a;
                ang_vel_a.0 += -Vec2::perp_dot(ra, impulse) * i_a;
                vel_b.0 += impulse * w_b;
                ang_vel_b.0 += Vec2::perp_dot(rb, impulse) * i_b;
            }
        }
    }
}
