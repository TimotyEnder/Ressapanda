use cgmath::{InnerSpace, Point3, SquareMatrix, Vector3, Vector4, vec3};

use crate::{
    camera::{Camera, OPENGL_TO_WGPU_MATRIX},
    voxel_instance::VoxelInstance,
    voxel_scene::VoxelScene,
};
#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
}
pub fn raycast_compute_from_mouse_position(
    camera: &Camera,
    mouse_x: f64,
    mouse_y: f64,
    view_port_width: f64,
    view_port_height: f64,
) -> Ray {
    //this is for me cuz linear algebra is fucking hard man
    //convert screen position into normalised device positions
    let ndc_x = (2.0 * mouse_x as f32 / view_port_width as f32) - 1.0;
    let ndc_y = 1.0 - (2.0 * mouse_y as f32 / view_port_height as f32);
    //ndc->clip space
    let near_point = Vector4::new(ndc_x, ndc_y, 0.0, 1.0);
    let far_point = Vector4::new(ndc_x, ndc_y, 1.0, 1.0);
    //clip space -> world space
    let inv_vp = (OPENGL_TO_WGPU_MATRIX * camera.build_view_projection_matrix()).invert();
    // perspective divide — multiply by inverse matrix, then divide by w
    let world_near = inv_vp.unwrap() * near_point;
    let world_near = world_near.truncate() / world_near.w;

    let world_far = inv_vp.unwrap() * far_point;
    let world_far = world_far.truncate() / world_far.w;

    // build the ray
    let origin = world_near;
    let direction = (world_far - world_near).normalize();
    Ray {
        origin: Point3::new(origin.x, origin.y, origin.z),
        direction,
    }
}
pub fn t_min_for_voxel_ray_overlap(ray: &Ray, voxel: &VoxelInstance) -> Option<f32> {
    let mut t_min = f32::NEG_INFINITY;
    let mut t_max = f32::INFINITY;
    let min = voxel.get_position() - vec3(0.5, 0.5, 0.5);
    let max = voxel.get_position() + vec3(0.5, 0.5, 0.5);
    for i in 0..3 {
        if ray.direction[i].abs() < 1e-8 {
            if ray.origin[i] < min[i] || ray.origin[i] > max[i] {
                return None;
            }
        } else {
            let t1 = (min[i] - ray.origin[i]) / ray.direction[i];
            let t2 = (max[i] - ray.origin[i]) / ray.direction[i];
            t_min = t_min.max(t1.min(t2));
            t_max = t_max.min(t1.max(t2));
            if t_min > t_max {
                return None;
            }
        }
    }
    if t_max < 0.0 {
        return None;
    }
    Some(if t_min > 0.0 { t_min } else { t_max })
}
pub fn find_first_voxel_to_intersect_ray(
    ray: Ray,
    voxel_scene: &VoxelScene,
) -> Option<(Point3<f32>, Vector3<f32>)> {
    let mut min_t_min: Option<f32> = None;
    let mut min_tmin_voxel = None;
    for voxel in voxel_scene.get_all_voxels() {
        if let Some(t_min) = t_min_for_voxel_ray_overlap(&ray, voxel) {
            if let Some(min_t_min) = min_t_min.as_mut() {
                *min_t_min = (*min_t_min).min(t_min);
                if *min_t_min == t_min {
                    min_tmin_voxel = Some(voxel);
                }
            } else {
                min_t_min = Some(t_min);
                min_tmin_voxel = Some(voxel);
            }
        }
    }
    if let Some(min_t_min) = min_t_min
        && let Some(voxel) = min_tmin_voxel
    {
        return Some((
            ray.origin + (ray.direction * min_t_min),
            voxel.get_position(),
        ));
    }
    return None;
}
pub fn find_pos_of_ray_vectors_closest_point_to_voxel_pos(
    ray: Ray,
    voxel_pos: Vector3<f32>,
) -> Point3<f32> {
    //damn linear alg is so cool!
    // t of point C that is closest to voxel P
    // t  = (P − O) · D / (D · D)
    // C  = O + t·D
    let t_closest_to_voxel = ((voxel_pos - vec3(ray.origin.x, ray.origin.y, ray.origin.z))
        .dot(ray.direction))
        / (ray.direction.dot(ray.direction));
    let closest_point_to_voxel_on_ray_vector = ray.origin + (t_closest_to_voxel * ray.direction);
    closest_point_to_voxel_on_ray_vector
}
