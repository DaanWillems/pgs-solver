use bevy::prelude::*;
use crate::components::*;
use crate::util::spawn;
use rand::Rng;

pub fn input_handler(mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut players: Query<(&Position, &mut Velocity), With<Player>>
) {

    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();

    let speed = 10.;
    let mut acceleration = Vec2::new(0., 0.);

    if keys.pressed(KeyCode::KeyW) {
        acceleration.y += speed;
    }

    if keys.pressed(KeyCode::KeyS) {
        acceleration.y -= speed;
    }

    if keys.pressed(KeyCode::KeyA) {
        acceleration.x -= speed;
    }

    if keys.pressed(KeyCode::KeyD) {
        acceleration.x += speed;
    }

    for (_, mut velocity) in players.iter_mut() {
        velocity.0 += acceleration;
    }

    if keys.pressed(KeyCode::Space) {
        if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            let mut rng = rand::thread_rng();
            // spawn::spawn_circle(&mut commands, &mut meshes, &mut materials, Vec2::new(world_position.x.clone(), world_position.y.clone()), Vec2::new(0.0, 0.0), 40., 20., false);
            spawn::spawn_rect_obb(&mut commands, &mut meshes, &mut materials, Vec2::new(world_position.x.clone(), world_position.y.clone()), Vec2::new(0.0, 0.0),  rng.gen_range(0.0..360.0), Vec2::new(80., 80.), 20., false);
            
        }
    }

    if keys.just_pressed(KeyCode::KeyQ) {
        if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            spawn::spawn_rect(&mut commands, &mut meshes, &mut materials, Vec2::new(world_position.x.clone(), world_position.y.clone()), Vec2::new(0.0, 0.0), Vec2::new(80., 80.), 20., false);
        }
    }
}