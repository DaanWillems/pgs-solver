use bevy::prelude::*;
use crate::components::*;
use crate::util::*;
use crate::resources::*;


pub fn find_collisions_test(mut rects: Query<(Entity, &mut Position, &Rotation, &ConvexCollider), Without<CircleCollider>>) {
    for (entity, pos, rot, collider) in rects.iter_mut() {

        let x_axis = (top_right - top_left).normalize();
        let y_axis = (top_right - bottom_right).normalize();

        for point in collider.points {
            point = 
        }



        //Calculate points
        let top_left = pos.0 + Vec2::new(-aabb.half_size.x, aabb.half_size.y);
        let top_right = pos.0 + Vec2::new(aabb.half_size.x, aabb.half_size.y);
        let bottom_right = pos.0 + Vec2::new(aabb.half_size.x, -aabb.half_size.y);
        let bottom_left = pos.0 + Vec2::new(-aabb.half_size.x, -aabb.half_size.y);

        //Calculate axis
        let x_axis = (top_right - top_left).normalize();
        let y_axis = (top_right - bottom_right).normalize();

        let proj_top_left = top_left.dot(x_axis);
        let proj_top_right = top_left.dot(x_axis);
        let proj_bottom_right = top_left.dot(x_axis);
        let proj_bottom_left = top_left.dot(x_axis);

        println!("point top left {}", top_left);
        println!("point top right {}", top_right);
        println!("point bottom right {}", bottom_right);
        println!("point bottom left {}", bottom_left);
    }
}

pub fn find_collisions(mut circles: Query<(Entity, &mut Position, &CircleCollider), Without<AABBCollider>>,
                    mut rects: Query<(Entity, &mut Position, &AABBCollider), Without<CircleCollider>>,
                    mut contacts: ResMut<Contacts>) {

    contacts.0.clear();
    let mut iter = rects.iter_combinations_mut();
    while let Some([(entity_a, pos_a, aabb_a), (entity_b, pos_b, aabb_b)]) =
        iter.fetch_next()
    {
        let contact_result = collisions::aabb_aabb(entity_a, pos_a.0, aabb_a, entity_b, pos_b.0, aabb_b);
        
        let contact = match contact_result {
            Some(contact) => contact,
            None => continue
        };

        contacts.0.push(contact);
    }

    let mut iter = circles.iter_combinations_mut();
    while let Some([(entity_a, pos_a, circle_a), (entity_b, pos_b, circle_b)]) =
        iter.fetch_next()
    {
        let contact_result = collisions::circle_circle(entity_a, pos_a.0, circle_a.radius, entity_b, pos_b.0, circle_b.radius);
        
        let contact = match contact_result {
            Some(contact) => contact,
            None => continue
        };
        contacts.0.push(contact);
    }

    for (circle_entity, circle_pos, circle_collider) in circles.iter_mut() {
        for (rect_entity, rect_pos, rect_collider) in rects.iter_mut() {
            let contact_result = collisions::circle_abbb(circle_entity, circle_pos.0, circle_collider.radius, rect_entity, rect_pos.0, rect_collider);
        
            let contact = match contact_result {
                Some(contact) => contact,
                None => continue
            };
            contacts.0.push(contact);
        }
    }
}
