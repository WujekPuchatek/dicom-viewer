use std::borrow::Cow;
use wgpu::{BindGroup, ComputePipeline, ShaderModule, Texture, TextureView};
use crate::rendering::compute_shaders::compute_shader::ComputeShader;
use crate::utils::data_dimensions::Dimensions;

pub struct RaycastCompute {
    shader: ShaderModule,
    bind_group: BindGroup,
    pipeline: ComputePipeline,
    work_group_count: (u32, u32, u32),

    normal_tex : Texture,
    normal_view : TextureView,
}

impl RaycastCompute {
    pub fn init(_adapter: &wgpu::Adapter,
                device: &wgpu::Device,
                _queue: &wgpu::Queue,
                dimensions: &Dimensions,
                data_tex_view: &TextureView) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Raycast compute"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("../shaders/raycast_compute.wgsl"))),
        });

        let normal_tex = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Raycast texture"),
            size: wgpu::Extent3d {
                width: dimensions.width,
                height: dimensions.height,
                depth_or_array_layers: dimensions.depth,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D3,
            format: wgpu::TextureFormat::Rgba32Float,
            usage: wgpu::TextureUsages::STORAGE_BINDING |
                wgpu::TextureUsages::COPY_SRC |
                wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let normal_view = normal_tex.create_view(&wgpu::TextureViewDescriptor::default());

        let input_entry = wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::StorageTexture {
                access: wgpu::StorageTextureAccess::ReadOnly,
                format: wgpu::TextureFormat::R32Float,
                view_dimension: wgpu::TextureViewDimension::D3,
            },
            count: None,
        };

        let output_entry = wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::StorageTexture {
                access: wgpu::StorageTextureAccess::WriteOnly,
                format: wgpu::TextureFormat::Rgba32Float,
                view_dimension: wgpu::TextureViewDimension::D3,
            },
            count: None,
        };

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Raycast compute bind group layout"),
            entries: &[input_entry, output_entry],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Raycast compute bind group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&data_tex_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&normal_view),
                },
            ],
        });

        let compute_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Raycast compute pipeline layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Raycast compute pipeline"),
            layout: Some(&compute_pipeline_layout),
            module: &shader,
            entry_point: "main",
        });

        Self {
            shader,
            bind_group,
            pipeline,
            work_group_count: Self::compute_work_group_count(
                (dimensions.width, dimensions.height, dimensions.depth),
                (8, 8, 4),
            ),
            normal_tex,
            normal_view,
        }
    }
}

impl ComputeShader for RaycastCompute {
    fn step(&self, _device: &wgpu::Device, _queue: &wgpu::Queue, encoder: &mut wgpu::CommandEncoder) {

        let (dispatch_width, dispatch_height, dispatch_depth) = self.work_group_count;
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Raycast compute pass"),
            timestamp_writes: None,
        });
        compute_pass.set_pipeline(&self.pipeline);
        compute_pass.set_bind_group(0, &self.bind_group, &[]);
        compute_pass.dispatch_workgroups(dispatch_width, dispatch_height, dispatch_depth);
    }
}