struct RescaleValues {
    slope: f32,
    intercept: f32,

    padding: vec2<f32>,
}

@group(0) @binding(0) var texture: texture_storage_3d<r32float, read_write>;
@group(0) @binding(1) var<storage> rescale_values: RescaleValues;

fn get_rescale_value(i: i32) -> f32 {
    return rescale_values[i];
}

@compute @workgroup_size(16, 16, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let value = textureLoad(texture, global_id).r;

    let scale_params = get_rescale_value(global_id.z);
    let rescaled = value * scale_params.slope + scale_params.intercept;

    textureStore(texture, global_id, rescaled);
}
