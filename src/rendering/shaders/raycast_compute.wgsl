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
@group(0) @binding(1) var output_texture: texture_storage_3d<rgba32float, write>;

@group(0) @binding(2) var<uniform> camera: Camera;
@group(0) @binding(3) var<uniform> model: Model;
@group(0) @binding(4) var<uniform> light: Light;

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {

}