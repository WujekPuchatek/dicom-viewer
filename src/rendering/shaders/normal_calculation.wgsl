@group(0) @binding(0) var input_texture: texture_storage_3d<r32float, read>;
@group(0) @binding(1) var output_texture: texture_storage_3d<rgba32float, write>;

fn uvec3_to_ivec3(v: vec3<u32>) -> vec3<i32> {
    return vec3<i32>(i32(v.x), i32(v.y), i32(v.z));
}

fn calculate_gradient(global_id: vec3<u32>, direction_vec: vec3<i32>) -> f32 {
    let id = uvec3_to_ivec3(global_id);

    let coord0 = id + vec3<i32>(-2, -2, -2) * direction_vec;
    let coord1 = id + vec3<i32>(-1, -1, -1) * direction_vec;

    let coord2 = id + vec3<i32>(1, 1, 1)    * direction_vec;
    let coord3 = id + vec3<i32>(2, 2, 2)    * direction_vec;


    let gradient =   1.0 * textureLoad(input_texture, coord0).r
                  - 8.0 * textureLoad(input_texture, coord1).r
                  + 8.0 * textureLoad(input_texture, coord2).r
                  - 1.0 * textureLoad(input_texture, coord3).r;

    return gradient / 12.0;
}

@compute @workgroup_size(8, 8, 4)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let dx = calculate_gradient(global_id, vec3<i32>(1, 0, 0));
    let dy = calculate_gradient(global_id, vec3<i32>(0, 1, 0));
    let dz = calculate_gradient(global_id, vec3<i32>(0, 0, 1));

    let gradient = normalize(vec3<f32>(dx, dy, dz));

    textureStore(output_texture, global_id, vec4<f32>(gradient, 1.0));
}
