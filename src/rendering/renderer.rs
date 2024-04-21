use std::borrow::Cow;
use web_sys::js_sys::DataView;
use wgpu::{BindGroup, ComputePipeline, ShaderModule, Texture, TextureView};
use crate::rendering::camera::CameraUniform;
use crate::utils::data_dimensions::Dimensions;

struct NormalRenderer {
    shader: ShaderModule,
    pipeline: ComputePipeline,
    bind_group: BindGroup,

    dimensions: Dimensions,

    data_tex: Texture,
    normal_tex : Texture,
}

impl NormalRenderer {
    fn init(_adapter: &wgpu::Adapter,
            device: &wgpu::Device,
            queue: &wgpu::Queue,
            dimensions: Dimensions,
            data_tex: TextureView) -> Self {

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
             label: Some("Compute normal"),
             source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders/normal_calculation.wgsl"))),
         });

        let normal_tex = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Normal texture"),
            size: wgpu::Extent3d {
                width: dimensions.width,
                height: dimensions.height,
                depth_or_array_layers: dimensions.depth,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D3,
            format: wgpu::TextureFormat::Rgba32Float,
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        let input_entry =

        let output_entry = wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::StorageTexture {
                access: wgpu::StorageTextureAccess::WriteOnly,
                format: wgpu::TextureFormat::Rgba32Float,
                view_dimension: wgpu::TextureViewDimension::D3,
            },
            count: None,
        }

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                camera_binding.bind_group_layout_entry(),
                model_binding.bind_group_layout_entry(),
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D3,
                    },
                    count: None,
                },
                sampler_binding.bind_group_layout_entry(),
            ],
        });

        Self { shader, dimensions, normal_tex, data_tex }
    }
}