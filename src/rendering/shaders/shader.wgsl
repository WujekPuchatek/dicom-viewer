struct VertexOutput {
    @location(0) tex_coord: vec3<f32>,
    @builtin(position) position: vec4<f32>,
};

@group(0)
@binding(0)
var<uniform> transform: mat4x4<f32>;

@vertex
fn vs_main(
    @location(0) position: vec4<f32>,
    @location(1) tex_coord: vec3<f32>,
) -> VertexOutput {
    var result: VertexOutput;
    result.tex_coord = tex_coord;
    result.position = transform * position;
    return result;
}

@group(0)
@binding(1)
var r_color: texture_3d<f32>;

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    let x = i32(vertex.tex_coord.x * 511);
    let y = i32(vertex.tex_coord.y * 511);
    let z = i32(vertex.tex_coord.z * 219);

    let tex = textureLoad(r_color, vec3<i32>(x, y, z), 0);
    let v = f32(tex.x) - 1024.0;

    let center = 40.0;
    let width = 400.0;
    let min = center - width / 2.0;

    let normalized = (v - min) / width;
    let saturated = saturate(normalized);

    return vec4<f32>(saturated, saturated, saturated, 1.0);
}

@fragment
fn fs_wire(vertex: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.5, 0.0, 0.5);
}