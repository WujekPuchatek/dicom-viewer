struct Camera {
	eye_pos: vec4<f32>,
	proj_view: mat4x4<f32>,
	inv_proj_view: mat4x4<f32>,
};

struct Model {
    transform: mat4x4<f32>,
    inv_transform: mat4x4<f32>,
};

struct VertexOutput {
    @location(0) tex_coord: vec3<f32>,
    @location(1) ray_dir: vec3<f32>,
    @location(2) transformed_eye: vec3<f32>,
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

fn calculate_value(v: f32) -> vec4<f32> {
    let center = 40.0;
    let width = 400.0;
    let min = center - width / 2.0;

    let normalized = (v - min) / width;
    let saturated = saturate(normalized);

    return vec4<f32>(saturated, saturated, saturated, 1.0);
}

fn mix(a: vec4<f32>, b: vec4<f32>, f: f32) -> vec4<f32> {
    return a * (1.0 - f) + b * f;
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

    result.position = camera.proj_view * model.transform * position;
    result.transformed_eye = camera.eye_pos.xyz;

    result.ray_dir = -camera.eye_pos.xyz;

    return result;
}

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    let ray_dir = normalize(vertex.ray_dir);

    let t = intersect_box(vertex.transformed_eye, ray_dir);
    let t_near = t.x;
    let t_far = t.y;

    if t_near > t_far {
        return vec4<f32>(0.0, 0.5, 0.5, 1.0);
    }

    let start_x = max(t_near, 0.0);
    let end_x = min(t_far, 1.0);

    let factory_opacity = 0.1;

    var color = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    let step = 0.01;

    for (var x = start_x; x < end_x; x += step) {
        let pos = vertex.transformed_eye + ray_dir * x;
        let tex = textureSampleLevel(hu_values, hu_sampler, pos, 0.0);

        let val = calculate_value(tex.x);

        color = mix(color, val, factory_opacity);

        if (color.a >= 0.95) {
            break;
        }
    }

    return vec4<f32>(color.xyz, 1.0);
}
