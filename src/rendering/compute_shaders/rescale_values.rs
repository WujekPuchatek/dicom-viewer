use std::borrow::Cow;
use bytemuck::{Pod, Zeroable};
use wgpu::{BindGroup, ComputePipeline, ShaderModule, TextureView};
use wgpu::util::DeviceExt;
use crate::examination::examination::Examination;
use crate::information_object_definitions::modality_lut::ModalityLut;
use crate::rendering::compute_shaders::compute_shader::ComputeShader;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct RescaleValues {
    slope: f32,
    intercept: f32,
    padding: [f32; 2],
}

impl RescaleValues {
    fn from_modality_lut(modality_lut: &ModalityLut) -> Self {
        Self {
            slope: modality_lut.rescale_slope,
            intercept: modality_lut.rescale_intercept,
            padding: [0.0; 2],
        }
    }
}

struct ComputeRescaleValues {
    shader: ShaderModule,
    bind_group: BindGroup,
    pipeline: ComputePipeline,
    work_group_count: (u32, u32, u32),
}

impl ComputeRescaleValues {
    pub fn init(_adapter: &wgpu::Adapter,
                device: &wgpu::Device,
                _queue: &wgpu::Queue,
                exam: Examination,
                data: TextureView) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Compute normal"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("../shaders/rescale_values.wgsl"))),
        });

        let texture_entry = wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::StorageTexture {
                access: wgpu::StorageTextureAccess::ReadWrite,
                format: wgpu::TextureFormat::R32Float,
                view_dimension: wgpu::TextureViewDimension::D3,
            },
            count: None,
        };

        let rescale_values = Self::get_rescale_values(&exam);

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Array Buffer"),
            contents: bytemuck::cast_slice(&rescale_values),
            usage: wgpu::BufferUsages::STORAGE,
        });

        let rescale_values_entry = wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: wgpu::BufferSize::new(4 * std::mem::size_of::<f32>() as u64),
            },
            count: None,
        };

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Rescale values bind group layout"),
            entries: &[texture_entry, rescale_values_entry],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Normal calculation bind group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&data),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: buffer.as_entire_binding(),
                },
            ],
        });

        let compute_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Rescale values pipeline layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Rescale values pipeline"),
            layout: Some(&compute_pipeline_layout),
            module: &shader,
            entry_point: "main",
        });

        let dimensions = exam.get_dimensions();

        Self {
            shader,
            bind_group,
            pipeline,
            work_group_count: Self::compute_work_group_count(
                (dimensions.width, dimensions.height, dimensions.depth),
                (16, 16, 1),
            ),
        }
    }

    fn get_rescale_values(exam: &Examination) -> Vec<RescaleValues> {
        let dicom_files = exam.get_dicom_files();

        dicom_files
            .iter()
            .map(|file|
                RescaleValues::from_modality_lut(&file.modality_lut))
            .collect::<Vec<_>>()
    }
}

impl ComputeShader for ComputeRescaleValues {
    fn step(&self, device: &wgpu::Device, queue: &wgpu::Queue) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Command Encoder for GPU compute"),
        });
        let (dispatch_width, dispatch_height, dispatch_depth) = self.work_group_count;
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Rescale values pass"),
            timestamp_writes: None,
        });
        compute_pass.set_pipeline(&self.pipeline);
        compute_pass.set_bind_group(0, &self.bind_group, &[]);
        compute_pass.dispatch_workgroups(dispatch_width, dispatch_height, dispatch_depth);
        drop(compute_pass);
        // Resolves any queries that might be in flight.
        queue.submit(std::iter::once(encoder.finish()));
    }
}