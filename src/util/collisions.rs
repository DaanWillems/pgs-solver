use std::u8::MAX;

use bevy::prelude::*;
use crate::{components, AABBCollider};

pub struct Contact {
    pub entity_a: Entity,
    pub entity_b: Entity,
    pub overlap: f32,
    pub normal: Vec2,
    pub bias: f32
}

impl Contact {
    fn new(entity_a: Entity, entity_b: Entity, overlap: f32, normal: Vec2) -> Contact {
        Contact {
            entity_a,
            entity_b,
            overlap,
            normal,
            bias: 0.
        }
    }

    pub fn set_bias(&mut self, bias: f32) {
        self.bias = bias;
    }
}

pub fn circle_circle(entity_a: Entity, pos_a: Vec2, radius_a: f32, entity_b: Entity, pos_b: Vec2, radius_b: f32) -> Option<Contact> {
    let ab = pos_b - pos_a;
    let ab_sqr_len = ab.length_squared();
    let combined_radius = radius_a + radius_b;

    if ab_sqr_len < combined_radius * combined_radius {
        let ab_length = ab_sqr_len.sqrt();
        let normal = (ab / ab_length);
        let penetration_depth = ab_length - combined_radius;

        return Some(Contact::new(
            entity_a,
            entity_b,
            -penetration_depth,
            normal
        ));
    }

    return None;
}

pub fn aabb_aabb_new(entity_a: Entity, pos_a: Vec2, aabb_a: &AABBCollider, entity_b: Entity, pos_b: Vec2, aabb_b: &AABBCollider) -> Option<Contact> {
    let normal;
    let mut n_x= Vec2::new(0., 0.);
    let mut n_y= Vec2::new(0., 0.);
    let mut pen_depth_x = 0.;
    let mut pen_depth_y = 0.;
    let overlap;

    let diff = pos_b - pos_a;

    let x_h = aabb_a.half_size.x + aabb_b.half_size.x;
    if diff.x.abs() < x_h {
        pen_depth_x = x_h - diff.x.abs();
        n_x = Vec2::new(diff.x, 0.).normalize();
    }

    let y_h = aabb_a.half_size.y + aabb_b.half_size.y;
    if diff.y.abs() < y_h {
        pen_depth_y = y_h - diff.y.abs();
        n_y = Vec2::new(0., diff.y).normalize();
    }

    if pen_depth_x == 0. || pen_depth_y == 0. {
        return None;
    }

    if pen_depth_x < pen_depth_y {
        normal = n_x;
        overlap = pen_depth_x;
    } else {
        normal = n_y;
        overlap = pen_depth_y
    }

    return Some(Contact::new(
        entity_a,
        entity_b,
        overlap,
        normal
    ));
}

pub fn aabb_aabb(entity_a: Entity, pos_a: Vec2, aabb_a: &AABBCollider, entity_b: Entity, pos_b: Vec2, aabb_b: &AABBCollider) -> Option<Contact> {
    //Initiate overlap
    let mut overlap = f32::INFINITY;
    let mut normal;

    //x axis
    let proj_min_a = pos_a - aabb_a.half_size; 
    let proj_max_a = pos_a + aabb_a.half_size; 

    let proj_min_b = pos_b - aabb_b.half_size; 
    let proj_max_b = pos_b + aabb_b.half_size; 

    if proj_min_a.x > proj_max_b.x || proj_max_a.x < proj_min_b.x {
        //No overlap on x axis we can exit early
        return None;
    } else {
        if proj_max_a.x - proj_min_b.x < proj_max_b.x - proj_min_a.x {
            overlap = proj_max_a.x - proj_min_b.x;
            normal = Vec2::new(1., 0.);
        } else {
            overlap = proj_max_b.x - proj_min_a.x;
            normal = Vec2::new(-1., 0.);
        }
    }

    if proj_min_a.y > proj_max_b.y || proj_max_a.y < proj_min_b.y {
        //No overlap on y axis we can exit early
        return None;
    } else {
        if proj_max_a.y - proj_min_b.y < proj_max_b.y - proj_min_a.y {
            overlap = proj_max_a.y - proj_min_b.y;
            normal = Vec2::new(0., 1.);
        } else {
            overlap = proj_max_b.y - proj_min_a.y;
            normal = Vec2::new(0., -1.);
        }
    }
    return Some(
        Contact::new(
            entity_a,
            entity_b,
            overlap,
            normal
        )
    );
}

pub fn circle_abbb(entity_circle: Entity, circle_pos: Vec2, circle_radius: f32, entity_rect: Entity, rect_pos: Vec2, rect_collider: &components::AABBCollider) -> Option<Contact> {
    let pos_diff = circle_pos - rect_pos;
    let clamped = pos_diff.clamp(-rect_collider.half_size, rect_collider.half_size) ;
    let closest = rect_pos + clamped;
    let difference = closest - circle_pos;
    if difference.length() < circle_radius {
        let penetration_depth;
        let mut n = Vec2::new(0., 0.);
        
        if clamped.x.abs() == clamped.y.abs() {
            penetration_depth = -(circle_radius - difference.length());
            n = -(circle_pos - closest).normalize();
        } else {
            if difference.x.abs() > difference.y.abs() {
                if difference.x < 0. {
                    penetration_depth = circle_radius - difference.x.abs();
                    n.x = -1.;
                } else {
                    penetration_depth = circle_radius - difference.x.abs();
                    n.x = 1.;
                }
            } else {
                if difference.y < 0. {
                    penetration_depth = circle_radius - difference.y.abs();
                    n.y = -1.;
                }
                else {
                    penetration_depth = circle_radius - difference.y.abs();
                    n.y = 1.;
                }
            }
        }

        return Some(Contact::new(
            entity_circle,
            entity_rect,
            penetration_depth,
            n
        ));    }
    return None;
}