@group(0) @binding(0) var texture: texture_3d<f32>;
@group(0) @binding(1) var<storage> slopes: array<f32>;
@group(0) @binding(2) var<storage> intercepts: array<f32>;

@compute @workgroup_size(16, 16, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let value = textureLoad(texture, global_id, 0).r;

    let slope = slopes[global_id.z];
    let intercept = intercepts[global_id.z];

    let rescaled = value * slope + intercept;

    textureStore(texture, global_id, vec4<f32>(rescaled, 0.0, 0.0, 1.0));
}
