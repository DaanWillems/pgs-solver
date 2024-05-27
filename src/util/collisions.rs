use std::u8::MAX;

use crate::{components, AABBCollider, ConvexCollider, SpawnDotEvent};
use bevy::prelude::*;

pub struct Contact {
    pub entity_a: Entity,
    pub entity_b: Entity,
    pub overlap: f32,
    pub normal: Vec2,
    pub bias: f32,
}

pub struct Manifold {
    pub entity_a: Entity,
    pub entity_b: Entity,
    pub overlap: f32,
    pub normal: Vec2,
    pub contact_points: Vec<Vec2>,
    pub bias: f32,
}

impl Manifold {
    pub fn set_bias(&mut self, bias: f32) {
        self.bias = bias;
    }
}

impl Contact {
    pub fn new(entity_a: Entity, entity_b: Entity, overlap: f32, normal: Vec2) -> Contact {
        Contact {
            entity_a,
            entity_b,
            overlap,
            normal,
            bias: 0.,
        }
    }

    pub fn set_bias(&mut self, bias: f32) {
        self.bias = bias;
    }
}

pub fn get_edges(vertices: &Vec<Vec2>) -> Vec<(Vec2, Vec2)> {
    let mut edges = Vec::new();

    for i in 0..vertices.len() {
        edges.push((vertices[i], vertices[(i + 1) % vertices.len()]));
    }

    return edges;
}

pub fn get_normals(edges: &Vec<(Vec2, Vec2)>) -> Vec<Vec2> {
    let mut normals = Vec::new();

    for edge in edges {
        normals.push((edge.0 - edge.1).normalize())
    }

    return normals;
}

pub fn point_segment_distance(a: Vec2, b: Vec2, p: &Vec2) -> (Vec2, f32) {
    let ab = b - a;
    let ap = *p - a;

    let proj = ap.dot(ab);

    let ab_length_squared = ab.length_squared();

    let d = proj / ab_length_squared;
    let cp;

    if d <= 0. {
        cp = a;
    } else if d >= 1. {
        cp = b;
    } else {
        cp = a + ab * d;
    }

    return (cp, Vec2::distance(*p, cp));
}

pub fn circle_circle(
    entity_a: Entity,
    pos_a: Vec2,
    radius_a: f32,
    entity_b: Entity,
    pos_b: Vec2,
    radius_b: f32,
) -> Option<Contact> {
    let ab = pos_b - pos_a;
    let ab_sqr_len = ab.length_squared();
    let combined_radius = radius_a + radius_b;

    if ab_sqr_len < combined_radius * combined_radius {
        let ab_length = ab_sqr_len.sqrt();
        let normal = (ab / ab_length);
        let penetration_depth = ab_length - combined_radius;

        return Some(Contact::new(entity_a, entity_b, -penetration_depth, normal));
    }

    return None;
}

pub fn aabb_aabb_sat(
    entity_a: Entity,
    pos_a: Vec2,
    aabb_a: &AABBCollider,
    entity_b: Entity,
    pos_b: Vec2,
    aabb_b: &AABBCollider,
) -> Option<Contact> {
    let normal;
    let mut n_x = Vec2::new(0., 0.);
    let mut n_y = Vec2::new(0., 0.);
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

    return Some(Contact::new(entity_a, entity_b, overlap, normal));
}

pub fn obb_obb(
    entity_a: Entity,
    pos_a: Vec2,
    rot_a: f32,
    mass_a: f32,
    collider_a: &ConvexCollider,
    entity_b: Entity,
    pos_b: Vec2,
    rot_b: f32,
    mass_b: f32,
    collider_b: &ConvexCollider,
) -> (Vec<Vec2>, Vec<Vec2>, Option<Manifold>) {
    let points_a = ConvexCollider::transform_points(&collider_a.points, pos_a, rot_a);
    let points_b = ConvexCollider::transform_points(&collider_b.points, pos_b, rot_b);

    let edges_a = get_edges(&points_a);
    let edges_b = get_edges(&points_b);

    let axes_a = get_normals(&edges_a);
    let axes_b = get_normals(&edges_b);

    let axes = [axes_a, axes_b].concat();

    let mut mtv = Vec2::new(0., 0.);
    let mut smallest_overlap = f32::INFINITY;

    //Do projections:
    for axis in axes {
        let mut min_a = f32::INFINITY;
        let mut max_a = -f32::INFINITY;

        for point in points_a.iter() {
            min_a = f32::min(min_a, point.dot(axis));
            max_a = f32::max(max_a, point.dot(axis));
        }

        let mut min_b = f32::INFINITY;
        let mut max_b = -f32::INFINITY;

        for point in points_b.iter() {
            min_b = f32::min(min_b, point.dot(axis));
            max_b = f32::max(max_b, point.dot(axis));
        }

        if min_a > max_b || max_a < min_b {
            return (points_a, points_b, None);
        }

        if min_a <= max_b {
            if max_b - min_a < smallest_overlap {
                smallest_overlap = max_b - min_a;
                mtv = -axis;
            }
        }

        if max_a > min_b {
            if max_a - min_b < smallest_overlap {
                smallest_overlap = max_a - min_b;
                mtv = axis;
            }
        }
    }

    //There is a collision, lets find the contact points
    let mut pos_a_collision = pos_a;
    let mut pos_b_collision = pos_b;

    if mass_b == 0. {
        pos_a_collision = pos_a - smallest_overlap * mtv;
    } else if mass_a == 0. {
        pos_b_collision = pos_b + smallest_overlap * mtv;
    } else {
        pos_b_collision = pos_b + smallest_overlap * mtv;
        pos_a_collision = pos_a - smallest_overlap * mtv;
    }

    let points_a_collision = ConvexCollider::transform_points(&collider_a.points, pos_a_collision, rot_a);
    let points_b_collision = ConvexCollider::transform_points(&collider_b.points, pos_b_collision, rot_b);

    let edges_a_collision = get_edges(&points_a_collision);
    let edges_b_collision = get_edges(&points_b_collision);

    let mut smallest_dist = f32::INFINITY;
    let mut contact_points = Vec::new();

    for p in points_a_collision.iter() {
        for v in edges_b_collision.iter() {
            let (cp, d) = point_segment_distance(v.0, v.1, p);
            if (d - smallest_dist).abs() < 0.2 {
                contact_points.push(cp);
                // println!("{}", (d - smallest_dist).abs());
            } else if d < smallest_dist {
                contact_points.clear();
                contact_points.push(cp);
                smallest_dist = d
            }
        }
    }

    // let mut smallest_dist_b = f32::INFINITY;

    for p in points_b_collision.iter() {
        for v in edges_a_collision.iter() {
            let (cp, d) = point_segment_distance(v.0, v.1, p);
            if (d - smallest_dist).abs() < 0.2 {
                contact_points.push(cp);
                // println!("{}", (d - smallest_dist).abs());
            } else if d < smallest_dist { 
                contact_points.clear();
                contact_points.push(cp);
                smallest_dist = d;
            }
        }
    }

    return (points_a, points_b, Some(Manifold {
        entity_a,
        entity_b,
        overlap: smallest_overlap.abs(),
        normal: mtv,
        contact_points,
        bias: 0.,
    }));
}

pub fn aabb_aabb(
    entity_a: Entity,
    pos_a: Vec2,
    aabb_a: &AABBCollider,
    entity_b: Entity,
    pos_b: Vec2,
    aabb_b: &AABBCollider,
) -> Option<Contact> {
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
    return Some(Contact::new(entity_a, entity_b, overlap, normal));
}

pub fn circle_abbb(
    entity_circle: Entity,
    circle_pos: Vec2,
    circle_radius: f32,
    entity_rect: Entity,
    rect_pos: Vec2,
    rect_collider: &components::AABBCollider,
) -> Option<Contact> {
    let pos_diff = circle_pos - rect_pos;
    let clamped = pos_diff.clamp(-rect_collider.half_size, rect_collider.half_size);
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
                } else {
                    penetration_depth = circle_radius - difference.y.abs();
                    n.y = 1.;
                }
            }
        }

        return Some(Contact::new(
            entity_circle,
            entity_rect,
            penetration_depth,
            n,
        ));
    }
    return None;
}
