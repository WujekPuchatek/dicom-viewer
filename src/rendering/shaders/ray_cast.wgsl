struct Camera {
	eye_pos: vec4<f32>,
	proj_view: mat4x4<f32>,
	inv_proj_view: mat4x4<f32>,
};

struct Model {
    transform: mat4x4<f32>,
    inv_transform: mat4x4<f32>,
};

struct Light {
    position: vec3<f32>,
    ambient: vec3<f32>,
    diffuse: vec3<f32>,
    specular: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
    @location(1) ray_dir: vec3<f32>,
    @location(2) tex_coord: vec3<f32>,
};

const AMBIENT_STRENGTH: f32 = 0.1;

fn aabbIntersect (origin: vec3<f32>, direction: vec3<f32>) -> vec2<f32> {
    let min = vec3<f32>(-1.0, -1.0, -0.56);
    let max = vec3<f32>(1.0, 1.0, 0.56);

    let tmin = (min - origin) / direction;
    let tmax = (max - origin) / direction;

    let t1 = min(tmin, tmax);
    let t2 = max(tmin, tmax);

    let t_near = max(max(t1.x, t1.y), t1.z);
    let t_far = min(min(t2.x, t2.y), t2.z);

    return vec2<f32>(t_near, t_far);
}

fn convert_range(old_value: f32, old_min: f32, old_max: f32, new_min: f32, new_max: f32) -> f32 {
    return (old_value - old_min) / (old_max - old_min) * (new_max - new_min) + new_min;
}

fn convert_vec3(old_vec: vec3<f32>) -> vec3<f32> {
    return vec3<f32>(
        convert_range(old_vec.x, -1.0, 1.0, 0.0, 1.0),
        convert_range(old_vec.y, -1.0, 1.0, 0.0, 1.0),
        convert_range(old_vec.z, -0.56, 0.56, 0.0, 1.0),
    );
}

fn convert_value(value: f32) -> f32 {
    let window_center = 40.0;
    let window_width = 400.0;

    let min_value = window_center - (window_width / 2.0);
    let max_value = window_center + (window_width / 2.0);

    let converted = convert_range(value, min_value, max_value, 0.0, 1.0);
    return saturate(converted);
}

fn raymarchHit (pos: vec3<f32>) -> vec4<f32> {
    let converted_pos = convert_vec3(pos);
    let texel = textureSampleLevel(hu_values, tex_sampler, converted_pos, 0.0);

    let value = convert_value(texel.r);
    return vec4<f32>(value);
}

fn random (seed: vec3<f32>) -> f32 {
    return fract(sin(dot(seed, vec3<f32>(12.9898, 78.233, 35.864))) *
        43758.5453123);
}

fn correct_gamma (color: vec4<f32>) -> vec4<f32> {
    let gamma = 2.2;
    let gamma_vec = vec4<f32>(gamma, gamma, gamma, 1.0);
    return pow(color, gamma_vec);
}

fn diffuse (pos: vec3<f32>) -> vec3<f32> {
    let light_dir = normalize(light.position - pos);

    let converted_pos = convert_vec3(pos);
    let normal = textureSampleLevel(normal_to_surface, tex_sampler, converted_pos, 0.0).rgb;

    let diff = max(dot(normal, light_dir), 0.0);
    return diff * light.diffuse;
}

@group(0) @binding(0) var<uniform> camera: Camera;
//@group(0) @binding(1) var<uniform> model: Model;
@group(0) @binding(2) var hu_values : texture_3d<f32>;
@group(0) @binding(3) var tex_sampler: sampler;
@group(0) @binding(4) var<uniform> light: Light;
@group(0) @binding(5) var normal_to_surface : texture_3d<f32>;

@vertex
fn vs_main(
    @location(0) position: vec4<f32>,
    @location(1) tex_coord: vec3<f32>,
) -> VertexOutput {
    var result: VertexOutput;

    result.world_pos = position.xyz;
    result.position = camera.proj_view * position;
    result.tex_coord = tex_coord;

    result.ray_dir = normalize(result.world_pos - camera.eye_pos.xyz);

    return result;
}

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    let light_color = vec3<f32>(1.0);
    let ambient_color = vec3<f32>(0.1);

    let direction = vertex.ray_dir;
    let position = vertex.world_pos;

    let t = aabbIntersect(position, direction);
    let t_near = t.x;
    let t_far = t.y;

    if (t_near > t_far) {
        return vec4<f32>(0.0);
    }

    let ambient = AMBIENT_STRENGTH * light.ambient;

    let steps = 15000;
    let step_size = 0.005;
    let factory_opacity = 0.96;

    var pos = position + direction * (t_near - random(position) * step_size);

    var result = vec4<f32>(0.0, 0.0, 0.0, 0.0);

    for (var t = t_near; t < t_far; t += step_size) {
        var src = raymarchHit(pos);

        var rgb = src.rgb;
        rgb *= src.a;

        src = vec4<f32>(rgb, src.a);

        src *= factory_opacity;

        result += (1.0 - result.a) * src;

        pos += direction * step_size;

        if (result.a > 0.95) {
            break;
        }
    }

    let corrected_gamma = correct_gamma(result);
    return corrected_gamma;
}
