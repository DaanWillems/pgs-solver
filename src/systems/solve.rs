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

        // let e = 0.9;
        // let rel_v = vel_b.0 - vel_a.0;

        // let j = (-(1.+e)*(rel_v.dot(contact.normal))) / w_sum;

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

        let e = 0.49;
        let rel_v = vel_b.0 - vel_a.0;
        let rel_normal_v = rel_v.dot(contact.normal);

        let ja_va = -contact.normal;
        let ja_vb = contact.normal;

        let jv = ja_va.dot(vel_a.0)+ja_vb.dot(vel_b.0);

        // let b = (-(1. / DELTA_TIME)*(contact.penetration_depth))+(e*(rel_normal_v));

        // let b = -(e * rel_normal_v); //Without baumgarte
        let b = -(1./DELTA_TIME) * contact.penetration_depth + e * rel_normal_v;
        let lambda = (-(jv + b)) * meff;

        vel_a.0 += lambda * ja_va*w_a;
        vel_b.0 += lambda * ja_vb*w_b;

        // //Compute position impulse for post projection
        // let positionImpulse = meff * (1. * contact.penetration_depth);

        // if mass_a.0 != 0. {
        //     pos_a.0 -= (positionImpulse * contact.normal) / mass_a.0;
        // }
        // if mass_b.0 != 0. {
        //     pos_b.0 += (positionImpulse * contact.normal) / mass_b.0;

        // }

    }
}