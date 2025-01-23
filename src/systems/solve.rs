use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

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

        let b = -(0.5 / time.delta_seconds()) * (f32::max(0., manifold.overlap-0.05));
        manifold.set_bias(b);
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
    mut manifolds: ResMut<Manifolds>,
) {
    let sf = 0.6;
    let df = 0.4;

    for _ in 0..16 {
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

                let e = 0.0;

                let rel_v =
                    (vel_b.0 + angular_linear_velocity_b) - (vel_a.0 + angular_linear_velocity_a);

                let rel_normal_v = rel_v.dot(manifold.normal);

                let denom = (w_a + w_b + ((ra_perp_n * ra_perp_n) * i_a) + ((rb_perp_n * rb_perp_n) * i_b));
                let mut j_n = (-(1. + e) * (rel_normal_v)) - manifold.bias;
                j_n /= denom;
                j_n /= manifold.contact_points.len() as f32;

                let new_accumulated_impulse = f32::max(0.0, manifold.accumulated_n_impulse + j_n);
                j_n = new_accumulated_impulse - manifold.accumulated_n_impulse;
                manifold.set_accumulated_normal_impulse(new_accumulated_impulse);

                //Compute tangent friction impulse
                let mut tangent =  rel_v - rel_v.dot(manifold.normal) * manifold.normal;

                if tangent.x.abs() < 0.0001 && tangent.y.abs() < 0.0001 {
                    let impulse = j_n * manifold.normal;
                    impulses.push((ra, rb, impulse));
                    continue;
                }

                tangent = tangent.normalize();
                let rel_tangent_v = rel_v.dot(tangent);


                let rta = Vec2::dot(ra_perp, tangent);
                let rtb = Vec2::dot(rb_perp, tangent);

                let denom = w_a + w_b + ((rta * rta) * i_a) + ((rtb * rtb) * i_b);


                let mut j_t = -rel_tangent_v / denom;
                j_t /= manifold.contact_points.len() as f32;
                let mut friction_impulse = Vec2::new(0., 0.);

                if j_t.abs() <= j_n * sf {
                    friction_impulse = j_t * tangent;
                } else {
                    friction_impulse = -j_n * tangent * df; 
                }

                let impulse = j_n * manifold.normal + friction_impulse;

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
}
