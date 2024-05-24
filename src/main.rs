use bevy::prelude::*;

mod components;
mod systems;
mod util;
mod resources;

use components::*;
use systems::integrate::*;
use systems::input::*;
use systems::collide::*;
use systems::events::*;
use systems::solve::*;
use systems::render::*;
use systems::spawn::*;

use resources::*;

use util::collisions::point_segment_distance;
use util::spawn;

pub const DELTA_TIME: f32 = 1. / 64.;

fn main() {

    // let t = point_segment_distance(Vec2::new(0., 0.), Vec2::new(0., 10.), &Vec2::new(5., 5.));
    // println!("{} {}", t.0, t.1);

    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Contacts>()
        .add_event::<SpawnDotEvent>()
        .add_systems(Startup, (spawn_camera, init_world))
        .add_systems(Update, (input_handler.before(integrate), 
                                                integrate.before(find_collisions).after(input_handler), 
                                                find_collisions.before(spawn_dot).after(integrate),
                                                spawn_dot.before(pre_step).after(find_collisions),
                                                pre_step.before(apply_impulses).after(spawn_dot),
                                                apply_impulses.before(project_positions).after(pre_step),
                                                project_positions.after(apply_impulses)))
        .run();
}

fn init_world(mut commands: Commands, 
              mut meshes: ResMut<Assets<Mesh>>, 
              mut materials: ResMut<Assets<ColorMaterial>>) {
    
    // for i in 0..10 {
    //     spawn::spawn_circle(&mut commands, &mut meshes, &mut materials, Vec2::new( -200. + ((i as f32) *100.), 0.), Vec2::new( 0., 0.), 30., 20., false);
    // }
    // spawn::spawn_rect_obb(&mut commands, &mut meshes, &mut materials, Vec2::new( 0., 0.), Vec2::new( 0., 0.), 0., Vec2::new(100., 100.), 20., false);
    // spawn::spawn_rect_obb(&mut commands, &mut meshes, &mut materials, Vec2::new( 200., 0.), Vec2::new( -80., 0.), 45., Vec2::new(100., 100.), 20., false);
    spawn::spawn_rect_obb(&mut commands, &mut meshes, &mut materials, Vec2::new( -500., 0.), Vec2::new( 0., 0.), 0., Vec2::new(40., 600.), 0., false);
    spawn::spawn_rect_obb(&mut commands, &mut meshes, &mut materials, Vec2::new( 500., 0.), Vec2::new( 0., 0.), 0., Vec2::new(40., 600.), 0., false);
    spawn::spawn_rect_obb(&mut commands, &mut meshes, &mut materials, Vec2::new( 0., -350.), Vec2::new( 0., 0.), 0., Vec2::new(1000., 40.), 0., false);
}

