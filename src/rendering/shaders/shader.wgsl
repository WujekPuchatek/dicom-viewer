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

@group(0)
@binding(2)
var r_normal: texture_3d<f32>;

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    let x = f32(vertex.tex_coord.x * 511.0);
    let y = f32(vertex.tex_coord.y * 511.0);
    let z = f32(vertex.tex_coord.z * 219.0);

    let tex = textureLoad(r_color, vec3<i32>(i32(x), i32(y), i32(z)), 0);
    let v = f32(tex.x) - 1024.0;

    let center = 40.0;
    let width = 400.0;
    let min = center - width / 2.0;

    let normalized = (v - min) / width;
    let saturated = saturate(normalized);

    if (saturated < 0.1 || x < 29 || x > 482 || y < 29 || y > 482) {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }

    let coord = vec3<f32>(x, y, z);
    let dxPos = dpdx(coord);
    let dyPos = dpdy(coord);
    let faceNormal = normalize(cross(dyPos, dxPos));

    return vec4<f32>(faceNormal * 0.5 + 0.5, 0.01);
}
