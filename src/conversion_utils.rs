pub fn spherical_to_cartesian(radius: f64, yaw: f64, pitch: f64) -> cgmath::Point3<f32> {
    let x = radius * pitch.cos() * yaw.sin();
    let y = radius * pitch.sin();
    let z = radius * pitch.cos() * yaw.cos();
    (x as f32, y as f32, z as f32).into()
}
