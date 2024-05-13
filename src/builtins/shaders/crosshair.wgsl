struct VertexOutput {
  @builtin(position) position : vec4<f32>,
  @location(0) fragCoord: vec4<f32>
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index : u32) -> VertexOutput {
  var positions = array<vec2<f32>, 3>(
    vec2<f32>(-1.0, -3.0),
    vec2<f32>(3.0, 1.0),
    vec2<f32>(-1.0, 1.0)
  );

  var output: VertexOutput;
  output.position = vec4<f32>(positions[vertex_index], 0.0, 1.0);
  output.fragCoord = output.position;
  return output;
}

struct CrosshairSettings {
    color: vec4<f32>,
    length: u32,
    thickness: u32,
    gap: u32,
    padding: u32,
};

@group(0) @binding(0)
var<uniform> settings: CrosshairSettings;

fn scale_length(value: u32) -> f32 {
    return f32(value) * 0.002;  // Example scale factor
}

fn scale_thickness(value: u32) -> f32 {
    return f32(value) * 0.001;  // Example scale factor
}

fn scale_gap(value: u32) -> f32 {
    return f32(value) * 0.0025;  // Example scale factor
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let length = scale_length(settings.length);
    let thickness = scale_thickness(settings.thickness);
    let gap = scale_gap(settings.gap);

    let x = in.fragCoord.x;
    let y = in.fragCoord.y;

    var color = vec4<f32>(0.0, 0.0, 0.0, 0.0);

    // Check if within vertical line bounds
    if (abs(x) < thickness && abs(y) > gap && abs(y) < length) {
        color = settings.color;  // Use the color from settings
    }
    
    // Check if within horizontal line bounds
    if (abs(y) < thickness && abs(x) > gap && abs(x) < length) {
        color = settings.color;  // Use the color from settings
    }

    return color;
}