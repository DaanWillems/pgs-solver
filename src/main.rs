use bevy::prelude::*;

mod components;
mod resources;
mod systems;
mod util;

use components::*;
use systems::collide::*;
use systems::events::*;
use systems::input::*;
use systems::integrate::*;
use systems::render::*;
use systems::solve::*;
use systems::spawn::*;

use resources::*;

use util::collisions::point_segment_distance;
use util::collisions::Manifold;
use util::spawn;

pub const DELTA_TIME: f32 = 1. / 64.;

fn main() {
    // let t = point_segment_distance(Vec2::new(0., 0.), Vec2::new(0., 10.), &Vec2::new(5., 5.));
    // println!("{} {}", t.0, t.1);

    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Manifolds>()
        .add_event::<SpawnDotEvent>()
        .add_systems(Startup, (spawn_camera, init_world))
        .add_systems(
            Update,
            (
                input_handler.before(integrate),
                integrate.before(find_collisions).after(input_handler),
                find_collisions.before(spawn_dot).after(integrate),
                spawn_dot.before(pre_step).after(find_collisions),
                pre_step.before(apply_impulses).after(spawn_dot),
                apply_impulses.before(project_positions).after(pre_step),
                project_positions.after(apply_impulses),
            ),
        )
        .run();
}

fn init_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // for i in 0..10 {
    //     spawn::spawn_circle(&mut commands, &mut meshes, &mut materials, Vec2::new( -200. + ((i as f32) *100.), 0.), Vec2::new( 0., 0.), 30., 20., false);
    // }
    // spawn::spawn_rect_obb(
    //     &mut commands,
    //     &mut meshes,
    //     &mut materials,
    //     Vec2::new(0., 0.),
    //     Vec2::new(0., 0.),
    //     0.0,
    //     20.0,
    //     Vec2::new(200., 100.),
    //     2000.,
    //     true,
    // );
    // spawn::spawn_rect_obb(&mut commands, &mut meshes, &mut materials, Vec2::new( 200., 0.), Vec2::new( -80., 0.), 45., Vec2::new(100., 100.), 20., false);
    // spawn::spawn_rect_obb(
    //     &mut commands,
    //     &mut meshes,
    //     &mut materials,
    //     Color::rgb(1., 1., 1.),
    //     Vec2::new(-550., 0.),
    //     Vec2::new(0., 0.),
    //     0.,
    //     0.,
    //     Vec2::new(40., 600.),
    //     0.,
    //     false,
    // );
    // spawn::spawn_rect_obb(
    //     &mut commands,
    //     &mut meshes,
    //     &mut materials,
    //     Color::rgb(1., 1., 1.),
    //     Vec2::new(550., 0.),
    //     Vec2::new(0., 0.),
    //     0.,
    //     0.,
    //     Vec2::new(40., 600.),
    //     0.,
    //     false,
    // );
    // spawn::spawn_rect_obb(
    //     &mut commands,
    //     &mut meshes,
    //     &mut materials,
    //     Color::rgb(1., 1., 1.),
    //     Vec2::new(-200., 150.),
    //     Vec2::new(0., 0.),
    //     0.,
    //     -20.,
    //     Vec2::new(600., 20.),
    //     0.,
    //     false,
    // );
    // spawn::spawn_rect_obb(
    //     &mut commands,
    //     &mut meshes,
    //     &mut materials,
    //     Color::rgb(1., 1., 1.),
    //     Vec2::new(200., -100.),
    //     Vec2::new(0., 0.),
    //     0.,
    //     20.,
    //     Vec2::new(600., 20.),
    //     0.,
    //     false,
    // );
    spawn::spawn_rect_obb(
        &mut commands,
        &mut meshes,
        &mut materials,
        Color::rgb(1., 1., 1.),
        Vec2::new(0., -350.),
        Vec2::new(0., 0.),
        0.,
        0.,
        Vec2::new(1400., 40.),
        0.,
        false,
    );
}
