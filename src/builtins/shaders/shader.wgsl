struct CameraUniform {
    view_proj: mat4x4<f32>,
    camera_pos: vec4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec4<f32>,
    @location(2) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.world_position = vec4<f32>(model.position, 1.0);
    out.world_normal = vec4<f32>(model.normal, 0.0);
    out.tex_coords = model.tex_coords;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    return out;
}

// @group(0) @binding(0)
// var t_diffuse: texture_2d<f32>;
// @group(0)@binding(1)
// var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var light_dir = normalize(camera.camera_pos - in.world_position);
    var intensity = max(dot(in.world_normal, light_dir), 0.0);
    return vec4<f32>(intensity, intensity, intensity, 1.0);
    // return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
