use memlib::math::{Vector3, Vector2};

pub fn world_to_screen(world_location: &Vector3, camera_pos: Vector3, screen_width: i32, screen_height: i32, fov: Vector2, matricies: [Vector3; 3]) -> Option<Vector2> {
    let local = world_location - camera_pos;
    let trans = Vector3 {
        x: Vector3::dot(&local, &matricies[1]),
        y: Vector3::dot(&local, &matricies[2]),
        z: Vector3::dot(&local, &matricies[0])
    };

    if trans.z < 0.01 {
        return None;
    }

    let x = (screen_width as f32 / 2.0) * (1.0 - (trans.x / fov.x / trans.z));
    let y = (screen_height as f32 / 2.0) * (1.0 - (trans.y / fov.y / trans.z));

    Some(Vector2{x, y})
}