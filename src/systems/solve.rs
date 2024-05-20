use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;

use super::integrate::DELTA_TIME;

pub fn solve_collisions(query: Query<(&mut Position, &mut Velocity, &Mass)>,
                             contacts: Res<Contacts>) {

        for contact in contacts.0.iter() {
            let ((mut pos_a, mut vel_a, mass_a),
                (mut pos_b, mut vel_b, mass_b)) =  unsafe {
            assert!(contact.entity_a != contact.entity_b);
                (
                    query.get_unchecked(contact.entity_a).unwrap(),
                    query.get_unchecked(contact.entity_b).unwrap()
                )
            };
            // let w_a = 1. / mass_a.0;
            // let w_b = 1. / mass_b.0;

            // let w_sum = w_a + w_b;
            // let meff = 1. / (w_a + w_b);

            // let e = 0.0;
            // let rel_v = vel_b.0 - vel_a.0;
            // let dv = rel_v.dot(contact.normal);

            // let b = (-0.5 / DELTA_TIME)*(contact.penetration_depth);

            // let j = (-(1.+e)*(dv+b)) * meff;

            // vel_a.0 -= (j * contact.normal)/mass_a.0;
            // vel_b.0 += (j * contact.normal)/mass_b.0;
            
            // let dir_correction = (contact.normal*contact.penetration_depth)/w_sum;
            
            // pos_a.0 -= dir_correction / mass_a.0;
            // pos_b.0 += dir_correction / mass_b.0;

            let w_a;
            let w_b;

            if mass_a.0 == 0. {
                w_a = 0.;
            } else {
                w_a = 1. / mass_a.0;
            }

            if mass_b.0 == 0. {
                w_b = 0.;
            } else {
                w_b = 1. / mass_b.0;
            }

            let meff = 1. / (w_a + w_b);
            println!("normal: {}", contact.normal);

            let e = 0.5;
            let rel_v = vel_b.0 - vel_a.0;
            println!("rel_v: {}", rel_v);

            let rel_normal_v = rel_v.dot(contact.normal);
            println!("rel_normal_v: {}", rel_normal_v);

            let ja_va = -contact.normal;
            println!("ja va: {}", ja_va);

            let ja_vb = contact.normal;
            println!("ja vb: {}", ja_vb);

            let jv = ja_va.dot(vel_a.0)+ja_vb.dot(vel_b.0);
            println!("meff: {}", meff);
            println!("normal: {}", contact.normal);
            println!("ja va: {}", ja_va);
            println!("ja vb: {}", ja_vb);
            println!("vel a: {}", vel_a.0);
            println!("vel b: {}", vel_b.0);
            println!("ja_va dot: {}", ja_va.dot(vel_a.0));
            println!("ja_vb dot: {}", ja_vb.dot(vel_b.0));
            println!("jv: {}", jv);

            // let b = (-(1. / DELTA_TIME)*(contact.penetration_depth))+(e*(rel_normal_v));
            let b = e*(rel_normal_v);
            println!("contact.penetration_depth: {}", contact.penetration_depth);
            println!("e*(rel_normal_v): {} * {}", e, rel_normal_v);

            // let b = -(0.2/DELTA_TIME) * contact.penetration_depth;

            // let b = e * rel_normal_v;
            let lambda = (-1.*(jv + b)) * meff;
            let delta = lambda * ja_va;
            println!("delta: {}", delta);

            let delta2 = delta * w_a;
            println!("delta 2 {}", delta2);

            vel_a.0 += delta2;
            println!("vel_a {}", vel_a.0);

            vel_b.0 += lambda * ja_vb*w_b;
        }
}
