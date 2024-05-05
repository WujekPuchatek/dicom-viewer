struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(1) tex_coord: vec3<f32>,
};

@group(0) @binding(0) var hu_values : texture_3d<f32>;
@group(0) @binding(1) var tex_sampler: sampler;

fn convert_range(old_value: f32, old_min: f32, old_max: f32, new_min: f32, new_max: f32) -> f32 {
    return (old_value - old_min) / (old_max - old_min) * (new_max - new_min) + new_min;
}

fn convert_value(value: f32) -> f32 {
    let window_center = 40.0;
    let window_width = 400.0;

    let min_value = window_center - (window_width / 2.0);
    let max_value = window_center + (window_width / 2.0);

    let converted = convert_range(value, min_value, max_value, 0.0, 1.0);
    return saturate(converted);
}


@vertex
fn vs_main(
    @location(0) position: vec4<f32>,
    @location(1) tex_coord: vec3<f32>,
) -> VertexOutput {
    var result: VertexOutput;

    result.position = position;
    result.tex_coord = tex_coord;
    return result;
}

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    if (vertex.tex_coord.z > 1.0) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }

    let tex_val = textureSampleLevel(hu_values, tex_sampler, vertex.tex_coord, 0.0).r;
    let val = convert_value(tex_val);
    return vec4<f32>(val, val, val, 1.0);
}