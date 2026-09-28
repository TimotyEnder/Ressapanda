use cgmath::{Vector3, vec3};

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
pub fn fill_line_from_a_to_b(a_op_pos: Vector3<f32>, b_op_pos: Vector3<f32>) -> Vec<Vector3<f32>> {
    //bresenham_line_algorithm in 3D! This is always fun to implement tbh ^u^
    let a_rounded = [
        a_op_pos.x.round() as i32,
        a_op_pos.y.round() as i32,
        a_op_pos.z.round() as i32,
    ];
    let b_rounded = [
        b_op_pos.x.round() as i32,
        b_op_pos.y.round() as i32,
        b_op_pos.z.round() as i32,
    ];

    let mut p = a_rounded;
    let mut delta = [0i32; 3];
    let mut abs_delta = [0i32; 3];
    let mut step = [0i32; 3];
    for i in 0..3 {
        delta[i] = b_rounded[i] - a_rounded[i];
        abs_delta[i] = delta[i].abs();
        step[i] = delta[i].signum();
    }

    //the longest axis drives the loop, the other two are the ones we can step
    let (major, u, v) = if abs_delta[0] >= abs_delta[1] && abs_delta[0] >= abs_delta[2] {
        (0, 1, 2)
    } else if abs_delta[1] >= abs_delta[2] {
        (1, 2, 0)
    } else {
        (2, 0, 1)
    };

    let steps = abs_delta[major];
    let mut err = [
        2 * abs_delta[u] - abs_delta[major],
        2 * abs_delta[v] - abs_delta[major],
    ];

    let mut filled_operating_positions = Vec::<Vector3<f32>>::with_capacity(steps as usize + 1);
    for _ in 0..=steps {
        filled_operating_positions.push(Vector3::new(p[0] as f32, p[1] as f32, p[2] as f32));

        if err[0] > 0 {
            err[0] -= 2 * abs_delta[major];
            p[u] += step[u];
        }
        if err[1] > 0 {
            err[1] -= 2 * abs_delta[major];
            p[v] += step[v];
        }
        p[major] += step[major];
        err[0] += 2 * abs_delta[u];
        err[1] += 2 * abs_delta[v];
    }
    filled_operating_positions
}
