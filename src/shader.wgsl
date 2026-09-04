struct Camera
{
    view_proj: mat4x4<f32>,
    position:vec4<f32>,
}
@group(0) @binding(0)
var<uniform> camera:Camera;

struct VertexInput
{
    @location(0) position:vec3<f32>,
    @location(1) normal:vec3<f32>,
}

struct VertexOutput{
    @builtin(position) clip_position:vec4<f32>,
    @location(0) color:vec4<f32>,
    @location(1) world_pos:vec4<f32>,
    @location(2) normal:vec3<f32>,
    @location(3) local_pos:vec3<f32>,
    @location(4) info_vec:vec4<f32>,
}
struct InstanceInput
{
    @location(4) instance_matrix_0: vec4<f32>,
    @location(5) instance_matrix_1: vec4<f32>,
    @location(6) instance_matrix_2: vec4<f32>,
    @location(7) instance_matrix_3: vec4<f32>,
    @location(8) color:vec4<f32>,
    @location(9) info_vec:vec4<f32>,
}
@vertex
fn vs_main(model:VertexInput,instance:InstanceInput)->VertexOutput
{
    let instance_matrix = mat4x4<f32>(
        instance.instance_matrix_0,
        instance.instance_matrix_1,
        instance.instance_matrix_2,
        instance.instance_matrix_3,
    );
    var out:VertexOutput;
    out.clip_position= camera.view_proj * instance_matrix * vec4<f32>(model.position, 1.0);
    out.world_pos=  instance_matrix * vec4<f32>(model.position, 1.0);
    out.color=instance.color;
    out.normal = mat3x3<f32>(instance_matrix[0].xyz, instance_matrix[1].xyz, instance_matrix[2].xyz) * model.normal;
    let m0 = instance.instance_matrix_0;
    let m1 = instance.instance_matrix_1;
    let m2 = instance.instance_matrix_2;
    let center = instance.instance_matrix_3.xyz;
    let inv_rot = mat3x3<f32>(
        vec3(m0.x, m1.x, m2.x),
        vec3(m0.y, m1.y, m2.y),
        vec3(m0.z, m1.z, m2.z),
    );
    out.local_pos = inv_rot * (out.world_pos.xyz - center);
    out.info_vec = instance.info_vec;
    return out;
}

@fragment
fn fs_main(in:VertexOutput)->@location(0) vec4<f32>
{
  let normal = normalize(in.normal);
  let light_dir = normalize(camera.position.xyz - in.world_pos.xyz);

  let diffuse = max(dot(normal, light_dir), 0.0);
  let ambient = 0.4;

  let lit_color = in.color.rgb * (ambient + diffuse * 0.6);
  var final_color=lit_color;
  //selection logic
  if in.info_vec.x>0.5
  {
    let s = abs(in.local_pos);
    let max_s= max(s.x,max(s.y,s.z));
    let mid_s= s.x+s.y+s.z-min(s.x,min(s.y,s.z))-max_s;
    let edge_dist=0.5-mid_s;
    let t = 1.0 - smoothstep(0.0, 0.03, edge_dist);
    final_color = mix(final_color, vec3(0.0, 0.0 , 0.0), t);
  }
  return vec4<f32>(final_color, in.color.a);
}
