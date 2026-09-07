use cgmath::Vector3;

pub fn spherical_to_cartesian(radius: f64, yaw: f64, pitch: f64) -> cgmath::Point3<f32> {
    let x = radius * pitch.cos() * yaw.sin();
    let y = radius * pitch.sin();
    let z = radius * pitch.cos() * yaw.cos();
    (x as f32, y as f32, z as f32).into()
}
pub fn snap_vector_to_flat_direction(vector: Vector3<f32>) -> Vector3<f32> {
    if vector.x.abs() > vector.z.abs() {
        return Vector3 {
            x: dumb_round(vector.x),
            y: 0.0,
            z: 0.0,
        };
    } else {
        return Vector3 {
            x: 0.0,
            y: 0.0,
            z: dumb_round(vector.z),
        };
    }
}
pub fn dumb_round(float: f32) -> f32 {
    if float > 0.0 {
        float.ceil()
    } else {
        float.floor()
    }
}
