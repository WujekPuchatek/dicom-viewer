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

@group(0) @binding(0) var input_texture: texture_storage_3d<r32float, read>;
@group(0) @binding(1) var output_texture_3d: texture_storage_3d<rgba32float, write>;
@group(0) @binding(2) var output_texture: texture_storage_2d<rgba32float, write>;

@group(0) @binding(2) var<uniform> camera: Camera;
@group(0) @binding(3) var<uniform> model: Model;
@group(0) @binding(4) var<uniform> light: Light;

fn aabbIntersect(ray_origin: vec3<f32>, ray_dir: vec3<f32>, box_min: vec3<f32>, box_max: vec3<f32>) -> vec2<f32> {
    let tmin = (box_min - ray_origin) / ray_dir;
    let tmax = (box_max - ray_origin) / ray_dir;

    let t1 = min(tmin, tmax);
    let t2 = max(tmin, tmax);

    let t_near = max(max(t1.x, t1.y), t1.z);
    let t_far = min(min(t2.x, t2.y), t2.z);

    return vec2<f32>(t_near, t_far);
}

fn screenToView(screen_pos: vec2<f32>, depth: f32) -> vec3<f32> {
    let ndc = vec3<f32>(screen_pos, depth) * 2.0 - 1.0;
    let view_pos = camera.inv_proj_view * vec4<f32>(ndc, 1.0);
    return view_pos.xyz / view_pos.w;
}

fn getRayDir(screen_pos: vec2<f32>, depth: f32) -> vec3<f32> {
    let view_pos = screenToView(screen_pos, depth);
    return normalize(view_pos - camera.eye_pos.xyz);
}

fn getScreenCoord(global_id: vec3<u32>) -> vec2<f32> {
    return vec2<f32>(global_id.xy) / vec2<f32>(textureDimensions(output_texture));
}

@compute @workgroup_size(16, 16, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // Get the screen position of the current pixel
    let screen_pos = getScreenCoord(global_id);
    let ray_dir = getRayDir(screen_pos, 1.0);

    // How to calculate the intersection of a ray with a 3D texture?

    // Calculate the intersection of the ray with the unit cube
    let t_near_far = aabbIntersect(camera.eye_pos.xyz, ray_dir, vec3<f32>(-1.0, -1.0, -1.0), vec3<f32>(1.0, 1.0, 1.0));
    let t_near = t_near_far.x;
    let t_far = t_near_far.y;

    if t_near < t_far {
        let hit_pos = camera.eye_pos.xyz + ray_dir * t_near;
        let hit_normal = normalize(hit_pos);

        let light_dir = normalize(light.position - hit_pos);
        let diffuse = max(dot(hit_normal, light_dir), 0.0) * light.diffuse;
        let specular = pow(max(dot(reflect(-light_dir, hit_normal), -ray_dir), 0.0), 32.0) * light.specular;

        let color = diffuse + specular;
        output_texture.write(vec3<f32>(color, 1.0), vec3<i32>(0, 0, 0));
    }


}