use bevy::prelude::*;

mod components;
mod systems;
mod util;
mod resources;

use components::*;
use systems::integrate;
use systems::integrate::*;
use systems::input::*;
use systems::collide::*;
use systems::solve::*;
use systems::render::*;
use resources::*;

use util::spawn;

pub const DELTA_TIME: f32 = 1. / 64.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Contacts>()
        .add_systems(Startup, (spawn_camera, init_world))
        .add_systems(Update, (input_handler.before(integrate), 
                                                integrate.before(find_collisions).after(input_handler), 
                                                find_collisions.before(apply_impulses).after(integrate),
                                                pre_step.before(apply_impulses).after(find_collisions),
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
    // spawn::spawn_rect(&mut commands, &mut meshes, &mut materials, Vec2::new( -200., 0.), Vec2::new( 0., 0.), Vec2::new(60., 60.), 20., false);
    // spawn::spawn_rect(&mut commands, &mut meshes, &mut materials, Vec2::new( 200., 0.), Vec2::new( 0., 0.), Vec2::new(40., 40.), 20., false);
    spawn::spawn_rect(&mut commands, &mut meshes, &mut materials, Vec2::new( -500., 0.), Vec2::new( 0., 0.), Vec2::new(40., 600.), 0., false);
    spawn::spawn_rect(&mut commands, &mut meshes, &mut materials, Vec2::new( 500., 0.), Vec2::new( 0., 0.), Vec2::new(40., 600.), 0., false);
    spawn::spawn_rect(&mut commands, &mut meshes, &mut materials, Vec2::new( 0., -350.), Vec2::new( 0., 0.), Vec2::new(1000., 40.), 0., false);
    // spawn::spawn_rect(&mut commands, &mut meshes, &mut materials, Vec2::new( 210., 200.), Vec2::new( 0., 0.), Vec2::new(100., 100.), 20., true);
}

