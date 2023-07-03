struct VertexInput {
  @location(0) position: vec3<f32>,
  @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) tex_coords: vec2<f32>,
}

struct FragmentOutput {
  @location(0) color: vec4<f32>,
}

@group(0) @binding(0) var display_texture: texture_2d<f32>;
@group(0) @binding(1) var display_sampler: sampler;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
  var out: VertexOutput;
  out.clip_position = vec4(in.position, 1.0);
  out.tex_coords = in.tex_coords;
  return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
  var pixel = textureSample(display_texture, display_sampler, in.tex_coords).r;
  var pixel_color = vec3(ceil(pixel));
  return vec4(pixel_color, 1.0);
}
