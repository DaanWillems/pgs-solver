use crate::components::*;
use crate::resources::*;
use crate::systems::events::SpawnDotEvent;
use crate::util::*;
use bevy::prelude::*;

use self::collisions::Manifold;

pub fn find_collisions(
    mut ev_spawn_dot: EventWriter<SpawnDotEvent>,
    mut circles: Query<(Entity, &mut Position, &CircleCollider), Without<AABBCollider>>,
    mut rects: Query<(Entity, &mut Position, &Rotation, &ConvexCollider, &Mass), Without<CircleCollider>>,
    mut manifolds: ResMut<Manifolds>,
) {
    manifolds.0.clear();
    let mut iter = rects.iter_combinations_mut();
    while let Some([(entity_a, mut pos_a, rot_a, aabb_a, mass_a), (entity_b, mut pos_b, rot_b, aabb_b, mass_b)]) =
        iter.fetch_next()
    {
        let manifold_result = collisions::obb_obb(
            entity_a, pos_a.0, rot_a.0, mass_a.0, aabb_a, entity_b, pos_b.0, rot_b.0, mass_b.0, aabb_b,
        );

        let manifold = match manifold_result.2 {
            Some(manifold) => manifold,
            None => continue,
        };

        for point in manifold.contact_points.iter() {
            ev_spawn_dot.send(SpawnDotEvent {
                pos: point.clone(),
                radius: 5.,
            });
        }

        // for point in manifold_result.0.iter() {
        //     ev_spawn_dot.send(SpawnDotEvent {
        //         pos: point.clone(),
        //         radius: 5.,
        //     });
        // }
        // for point in manifold_result.1.iter() {
        //     ev_spawn_dot.send(SpawnDotEvent {
        //         pos: point.clone(),
        //         radius: 5.,
        //     });
        // }

        manifolds.0.push(manifold);
    }

    let mut iter = circles.iter_combinations_mut();
    while let Some([(entity_a, pos_a, circle_a), (entity_b, pos_b, circle_b)]) = iter.fetch_next() {
        let contact_result = collisions::circle_circle(
            entity_a,
            pos_a.0,
            circle_a.radius,
            entity_b,
            pos_b.0,
            circle_b.radius,
        );

        let contact = match contact_result {
            Some(contact) => contact,
            None => continue,
        };
        // contacts.0.push(contact);
    }

    // for (circle_entity, circle_pos, circle_collider) in circles.iter_mut() {
    //     for (rect_entity, rect_pos, rect_collider) in rects.iter_mut() {
    //         let contact_result = collisions::circle_abbb(circle_entity, circle_pos.0, circle_collider.radius, rect_entity, rect_pos.0, rect_collider);

    //         let contact = match contact_result {
    //             Some(contact) => contact,
    //             None => continue
    //         };
    //         contacts.0.push(contact);
    //     }
    // }
}
