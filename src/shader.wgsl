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
}
struct InstanceInput
{
    @location(4) instance_matrix_0: vec4<f32>,
    @location(5) instance_matrix_1: vec4<f32>,
    @location(6) instance_matrix_2: vec4<f32>,
    @location(7) instance_matrix_3: vec4<f32>,
    @location(8) color:vec4<f32>,
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
  return vec4<f32>(lit_color, in.color.a);
}
