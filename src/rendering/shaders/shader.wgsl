struct Camera {
	view_pos: vec4<f32>,
	proj_view: mat4x4<f32>,
	inv_proj: mat4x4<f32>,
};

struct Model {
    transform: mat4x4<f32>,
    inv_transform: mat4x4<f32>,
};

struct VertexOutput {
    @location(0) tex_coord: vec3<f32>,
    @builtin(position) position: vec4<f32>,
};

// https://michvalwin.com/posts/2023/04/26/ray-collisions.html
fn intersect_box(orig: vec3<f32>, dir: vec3<f32>) -> vec2<f32> {
    let box_min = vec3<f32>(0.0);
    let box_max = vec3<f32>(1.0);

    let inv_dir = 1.0 / dir;

    let tmin_tmp = (box_min - orig) * inv_dir;
    let tmax_tmp = (box_max - orig) * inv_dir;

    // In case of negative values, we need to swap them
    let t_min = min(tmin_tmp, tmax_tmp);
    let t_max = max(tmin_tmp, tmax_tmp);

    let t_near = max(t_min.x, max(t_min.y, t_min.z));
    let t_far = min(t_max.x, min(t_max.y, t_max.z));

    return vec2<f32>(t_near, t_far);
}

@group(0)
@binding(0)
var<uniform> camera: Camera;

@group(0)
@binding(1)
var<uniform> model: Model;

@group(0)
@binding(2)
var hu_values : texture_3d<f32>;

@group(0)
@binding(3)
var hu_sampler: sampler;

@vertex
fn vs_main(
    @location(0) position: vec4<f32>,
    @location(1) tex_coord: vec3<f32>,
) -> VertexOutput {
    var result: VertexOutput;

    result.tex_coord = tex_coord;
    result.position = model.transform * camera.proj_view  * position;

    return result;
}

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    let x = f32(vertex.tex_coord.x * 511.0);
    let y = f32(vertex.tex_coord.y * 511.0);
    let z = f32(vertex.tex_coord.z * 219.0);

    return vec4<f32>( f32(vertex.tex_coord.x), f32(vertex.tex_coord.y), f32(vertex.tex_coord.z), 1.0);

//    let tex = textureLoad(hu_values, vec3<i32>(i32(x), i32(y), i32(z)), 0);
//    let v = f32(tex.x) - 1024.0;
//
//    let center = 40.0;
//    let width = 400.0;
//    let min = center - width / 2.0;
//
//    let normalized = (v - min) / width;
//    let saturated = saturate(normalized);
//
//    return vec4<f32>(saturated, saturated, saturated, 1);
}
