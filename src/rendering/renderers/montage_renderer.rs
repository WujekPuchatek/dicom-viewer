use std::borrow::Cow;
use std::mem;
use wgpu::{Adapter, BindGroup, BindingType, Buffer, CommandEncoder, Device, Queue, RenderPipeline, SurfaceConfiguration, TextureView};
use wgpu::util::DeviceExt;
use crate::examination::examination::Examination;
use crate::rendering::renderers::renderer::Renderer;
use crate::rendering::sampler::{Sampler, SamplerBinding};
use crate::rendering::vertex::{Vertex, vertex_2d};
use crate::utils::data_dimensions::Dimensions;

struct VerticesParams {
    pos_x_start: f32,
    pos_x_step: f32,
    pos_y_start: f32,
    pos_y_step: f32,
    tex_coord_z_start: f32,
    tex_coord_z_step: f32,
}

fn construct_vertices_params(data_dims: &Dimensions,
                             num_of_rows: u16,
                             num_of_cols: u16,
                             first_slice_idx: u16,
                             aspect_ratio : f32) -> VerticesParams {
    let (image_width, image_height) = (data_dims.width as f32, data_dims.height as f32);

    let ratio = aspect_ratio * (num_of_rows as f32 / num_of_cols as f32) * (image_width / image_height);

    let (pos_x_start, pos_x_step) = {
        let ratio = ratio.max(1.0);

        let width = 2.0 / ratio;
        let shift = (2.0 - width) / 2.0;

        let pos_x_start = -1.0 + shift;
        let pos_x_step = width / num_of_cols as f32;

        (pos_x_start, pos_x_step)
    };

    let (pos_y_start, pos_y_step) = {
        let ratio = 1.0;

        let height = 2.0 * ratio;
        let shift = (2.0 - height) / 2.0;

        let pos_y_start = 1.0 - shift;
        let pos_y_step = - height / num_of_rows as f32;

        (pos_y_start, pos_y_step)
    };

    let tex_coord_z_step = 1.0 / data_dims.depth as f32;
    let tex_coord_z_start = (0.5 * first_slice_idx as f32) * tex_coord_z_step;

    VerticesParams {
        pos_x_start,
        pos_x_step,
        pos_y_start,
        pos_y_step,
        tex_coord_z_start,
        tex_coord_z_step,
    }
}
pub struct MontageRenderer {
    data_dims: Dimensions,

    vertex_buf: Buffer,
    index_buf: Buffer,
    index_count: usize,
    bind_group: BindGroup,
    pipeline: RenderPipeline,

    aspect_ratio : f32,
    num_of_cols : u16,
    num_of_rows : u16,
    first_slice_idx : u16,
}

fn create_vertices(data_dims: &Dimensions,
                   num_of_rows: u16,
                   num_of_cols: u16,
                   first_slice_idx: u16,
                   aspect_ratio : f32) -> (Vec<Vertex>, Vec<u16>) {
    let num_of_slices = num_of_rows * num_of_cols;

    let VerticesParams {
        pos_x_start,
        pos_x_step,
        pos_y_start,
        pos_y_step,
        tex_coord_z_start,
        tex_coord_z_step,
    } = construct_vertices_params(data_dims, num_of_rows, num_of_cols, first_slice_idx, aspect_ratio);

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for slice in 0..num_of_slices {
        let row = slice / num_of_cols;
        let col = slice % num_of_cols;

        let pos_x = pos_x_start + pos_x_step * col as f32;
        let pos_y = pos_y_start + pos_y_step * row as f32;
        let tex_coord_z = tex_coord_z_start + tex_coord_z_step * slice as f32;

        let vertex_index = vertices.len() as u16;

        vertices.push(vertex_2d([pos_x, pos_y + pos_y_step], [0.0, 0.0, tex_coord_z]));
        vertices.push(vertex_2d([pos_x + pos_x_step, pos_y + pos_y_step], [1.0, 0.0, tex_coord_z]));
        vertices.push(vertex_2d([pos_x, pos_y], [0.0, 1.0, tex_coord_z]));
        vertices.push(vertex_2d([pos_x + pos_x_step, pos_y], [1.0, 1.0, tex_coord_z]));

        indices.push(vertex_index);
        indices.push(vertex_index + 1);
        indices.push(vertex_index + 2);
        indices.push(vertex_index + 2);
        indices.push(vertex_index + 1);
        indices.push(vertex_index + 3);
    }
    (vertices, indices)
}

fn create_vertex_and_index_buffers(
    device: &Device,
    data_dims: &Dimensions,
    num_of_rows: u16,
    num_of_cols: u16,
    first_slice_idx: u16,
    aspect_ratio : f32) -> (Buffer, Buffer, usize) {
    let (vertex_data, index_data) =
        create_vertices(&data_dims, num_of_rows, num_of_cols, first_slice_idx, aspect_ratio);

    let vertex_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertex_data),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let index_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(&index_data),
        usage: wgpu::BufferUsages::INDEX,
    });

    (vertex_buf, index_buf, index_data.len())
}
impl MontageRenderer {
    pub fn init(
        config: &SurfaceConfiguration,
        _adapter: &Adapter,
        device: &Device,
        _queue: &Queue,
        exam: &Examination,
        data_view: &TextureView,
    ) -> Self {
        let data_dims = exam.get_dimensions();
        let num_of_cols = 20;
        let num_of_rows = 12;
        let first_slice_idx = 0;

        let aspect_ratio = config.width as f32 / config.height as f32;

        let vertex_size = mem::size_of::<Vertex>();
        let (vertex_buf, index_buf, index_count) =
            create_vertex_and_index_buffers(device, &data_dims, num_of_rows, num_of_cols, first_slice_idx, aspect_ratio);

        let sampler_binding = SamplerBinding::new(device, 1, Sampler::new());

        // Create pipeline layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D3,
                    },
                    count: None,
                },
                sampler_binding.bind_group_layout_entry(),
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&data_view),
                },
                sampler_binding.bind_group_entry(),
            ],
            label: None,
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("../shaders/montage.wgsl"))),
        });

        let vertex_buffers = [wgpu::VertexBufferLayout {
            array_stride: vertex_size as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 4 * 4,
                    shader_location: 1,
                },
            ],
        }];

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &vertex_buffers,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(config.view_formats[0].into())],
            }),
            primitive: wgpu::PrimitiveState {
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            multisample: wgpu::MultisampleState::default(),
            depth_stencil: None,
            multiview: None,
        });

        let aspect_ratio = config.width as f32 / config.height as f32;

        Self {
            data_dims,
            vertex_buf,
            index_buf,
            index_count,
            bind_group,
            pipeline,
            aspect_ratio,
            num_of_cols,
            num_of_rows,
            first_slice_idx,
        }
    }

    pub fn resize(
        &mut self,
        config: &SurfaceConfiguration,
        device: &Device,
        _queue: &Queue,
    ) {
        self.aspect_ratio = config.width as f32 / config.height as f32;
        (self.vertex_buf, self.index_buf, self.index_count) =
            create_vertex_and_index_buffers(device, &self.data_dims, self.num_of_rows, self.num_of_cols, self.first_slice_idx, self.aspect_ratio);
    }

    pub fn update(&mut self, _event: winit::event::WindowEvent) {

    }
}

impl Renderer for MontageRenderer {
    fn render(&mut self,
              view: &TextureView,
              _device: &Device,
              _queue: &Queue,
              encoder: &mut CommandEncoder)
    {
        encoder.push_debug_group("montage render pass");
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.set_index_buffer(self.index_buf.slice(..), wgpu::IndexFormat::Uint16);
            rpass.set_vertex_buffer(0, self.vertex_buf.slice(..));
            rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);
        }
        encoder.pop_debug_group();
    }
}