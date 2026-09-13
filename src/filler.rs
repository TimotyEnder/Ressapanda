use cgmath::Vector3;

pub fn fill_positions_from_a_to_b(
    a_op_pos: Vector3<f32>,
    b_op_pos: Vector3<f32>,
) -> Vec<Vector3<f32>> {
    let mut filled_operating_positions = Vec::<Vector3<f32>>::new();
    for x in a_op_pos.x.min(b_op_pos.x) as i32..=a_op_pos.x.max(b_op_pos.x) as i32 {
        for y in a_op_pos.y.min(b_op_pos.y) as i32..=a_op_pos.y.max(b_op_pos.y) as i32 {
            for z in a_op_pos.z.min(b_op_pos.z) as i32..=a_op_pos.z.max(b_op_pos.z) as i32 {
                filled_operating_positions.push(Vector3::new(x as f32, y as f32, z as f32));
            }
        }
    }
    filled_operating_positions
}
