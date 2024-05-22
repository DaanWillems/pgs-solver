use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;

use super::integrate::DELTA_TIME;

pub fn pre_step(query: Query<(&mut Position, &mut Velocity, &Mass)>,
                             mut contacts: ResMut<Contacts>) {

        for contact in contacts.0.iter_mut() {
            let ((mut pos_a, mut vel_a, mass_a),
                (mut pos_b, mut vel_b, mass_b)) =  unsafe {
            assert!(contact.entity_a != contact.entity_b);
                (
                    query.get_unchecked(contact.entity_a).unwrap(),
                    query.get_unchecked(contact.entity_b).unwrap()
                )
            };
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
 
            let b = -(0.5/ DELTA_TIME)*(f32::max(0., contact.overlap-0.1));
            contact.set_bias(b);
            let e = 0.0;

            let ja_va = -contact.normal;
            let ja_vb = contact.normal;

            let rel_v = vel_b.0 - vel_a.0;

            let jv = ja_va.dot(vel_a.0)+ja_vb.dot(vel_b.0);

            let rel_normal_v = rel_v.dot(contact.normal);

            let restitution = e*(rel_normal_v);
            let lambda = (-1.*(jv + restitution + contact.bias)) * meff;

            vel_a.0 += lambda * ja_va*w_a;
            vel_b.0 += lambda * ja_vb*w_b;

        }
}

pub fn apply_impulses(query: Query<(&mut Position, &mut Velocity, &Mass)>,
                             contacts: Res<Contacts>) {
    for _ in 0..2 {
        for contact in contacts.0.iter() {
            let ((mut pos_a, mut vel_a, mass_a),
                (mut pos_b, mut vel_b, mass_b)) =  unsafe {
            assert!(contact.entity_a != contact.entity_b);
                (
                    query.get_unchecked(contact.entity_a).unwrap(),
                    query.get_unchecked(contact.entity_b).unwrap()
                )
            };
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

            let e = 0.0;
            let rel_v = vel_b.0 - vel_a.0;

            let rel_normal_v = rel_v.dot(contact.normal);
            
            if rel_normal_v > 0. {
                continue;
            }

            let ja_va = -contact.normal;
            let ja_vb = contact.normal;

            let jv = ja_va.dot(vel_a.0)+ja_vb.dot(vel_b.0);

            let restitution = e*(rel_normal_v);
            let lambda = (-1.*(jv + restitution + contact.bias)) * meff;

            vel_a.0 += lambda * ja_va*w_a;
            vel_b.0 += lambda * ja_vb*w_b;

        }
    }
}
