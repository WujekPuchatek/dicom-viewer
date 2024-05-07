use std::borrow::Cow;
use std::mem;
use bytemuck::{Pod, Zeroable};
use glam::Quat;
use wgpu::BindingType;
use wgpu::naga::SwizzleComponent::W;
use wgpu::util::DeviceExt;
use winit::event::WindowEvent;
use winit::keyboard::{Key, NamedKey, NativeKey};
use winit::keyboard::Key::Named;
use crate::examination::examination::Examination;
use crate::rendering::camera::{Camera, CameraBinding};
use crate::rendering::light::{Light, LightBinding};
use crate::rendering::model::{Model, ModelBinding};
use crate::rendering::renderers::renderer::Renderer;
use crate::rendering::sampler::{Sampler, SamplerBinding};
use crate::rendering::vertex::{Vertex, vertex};
use crate::utils::data_dimensions::Dimensions;


fn create_vertices(normalized_dims: [f32; 3]) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertex_data = [
        vertex([-1,-1, 1], [0, 0, 0]), // 0
        vertex([ 1,-1, 1], [1, 0, 0]), // 1
        vertex([-1,-1,-1], [0, 0, 1]), // 2
        vertex([ 1,-1,-1], [1, 0, 1]), // 3
        vertex([-1, 1, 1], [0, 1, 0]), // 4
        vertex([ 1, 1, 1], [1, 1, 0]), // 5
        vertex([-1, 1,-1], [0, 1, 1]), // 6
        vertex([ 1, 1,-1], [1, 1, 1]), // 7

    ];

    vertex_data.
        iter_mut().
        for_each(|v| {
            v.pos[0] *= normalized_dims[0];
            v.pos[1] *= normalized_dims[1];
            v.pos[2] *= normalized_dims[2];
        });

    let index_data: &[u16] = &[
        2, 1, 0, 2, 3, 1, // front
        0, 5, 4, 0, 1, 5, // top
        2, 6, 7, 2, 7, 3, // bottom
        4, 5, 6, 6, 5, 7, // back
        0, 4, 6, 0, 6, 2, // left
        5, 1, 3, 7, 5, 3, // right

    ];

    (vertex_data.to_vec(), index_data.to_vec())
}

pub struct RayCastRenderer {
    model: Model,
    light: Light,
    camera: Camera,

    data_dims: Dimensions,

    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
    index_count: usize,
    bind_group: wgpu::BindGroup,
    light_binding: LightBinding,
    model_binding: ModelBinding,
    camera_binding: CameraBinding,
    pipeline: wgpu::RenderPipeline,
}

impl RayCastRenderer {
    pub fn optional_features() -> wgpu::Features {
        wgpu::Features::POLYGON_MODE_LINE
    }

    pub fn init(
        config: &wgpu::SurfaceConfiguration,
        _adapter: &wgpu::Adapter,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        exam: &Examination,
        data_view: &wgpu::TextureView,
        normal_view: &wgpu::TextureView,
    ) -> Self {
        let data_dims = exam.get_dimensions();

        let mut light = Light::new();
        let mut model = Model::new(Quat::IDENTITY);
        let mut camera = Camera::new(config.width as f32 / config.height as f32);

        // Create the vertex and index buffers
        let vertex_size = mem::size_of::<Vertex>();
        let (vertex_data, index_data) = create_vertices(data_dims.get_normalized_dimensions());

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

        let mut camera_binding = CameraBinding::new(device, 0);
        let mut model_binding = ModelBinding::new(device, 1);
        let sampler_binding = SamplerBinding::new(device, 3, Sampler::new());
        let mut light_binding = LightBinding::new(device, 4);

        // Create pipeline layout
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
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D3,
                    },
                    count: None,
                },
                sampler_binding.bind_group_layout_entry(),
                light_binding.bind_group_layout_entry(),
                wgpu::BindGroupLayoutEntry {
                    binding: 5,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D3,
                    },
                    count: None,
                }
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        light_binding.update(queue, &mut light);
        model_binding.update(queue, &mut model);
        camera_binding.update(queue, &mut camera);

        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                camera_binding.bind_group_entry(),
                model_binding.bind_group_entry(),
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&data_view),
                },
                sampler_binding.bind_group_entry(),
                light_binding.bind_group_entry(),
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: wgpu::BindingResource::TextureView(&normal_view),
                }
            ],
            label: None,
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("../shaders/ray_cast.wgsl"))),
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

        RayCastRenderer {
            model,
            camera,
            light,
            data_dims,
            vertex_buf,
            index_buf,
            index_count: index_data.len(),
            bind_group,
            light_binding,
            model_binding,
            camera_binding,
            pipeline,
        }
    }

    pub fn resize(
        &mut self,
        config: &wgpu::SurfaceConfiguration,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
    ) {
        self.camera.set_aspect(config.width, config.height);
    }

    pub fn update(&mut self, event: WindowEvent) {
        println!("Event: {:?}", event);

        if let WindowEvent::KeyboardInput { event, .. } = event {
            if event.state == winit::event::ElementState::Pressed {
                match event.logical_key {
                    Key::Character(s)=> println!("Character: {}", s),
                    _ => {}
                }
            }
        }
    }

    pub fn update_zoom(&mut self, zoom_delta: f32, _queue: &wgpu::Queue) {
        const ZOOM_SPEED: f32 = 0.08;
        let zoom_delta = -zoom_delta * ZOOM_SPEED;

        self.camera.zoom_delta(zoom_delta);
    }

    pub fn rotate(&mut self, dx: f32, dy: f32, _queue: &wgpu::Queue) {

        const SENSITIVITY: f32 = 0.005;

        let dx = -dx * SENSITIVITY;
        let dy = dy * SENSITIVITY;

        self.camera.rotate(dx, dy);
    }

    pub fn move_forward(&mut self, delta: f32) {
        self.camera.move_forward(delta);
    }

    pub fn move_right(&mut self, delta: f32) {
        self.camera.move_right(delta);
    }

    pub fn move_up(&mut self, delta: f32) {
        self.camera.move_up(delta);
    }
}

impl Renderer for RayCastRenderer {
    fn render(&mut self,
              view: &wgpu::TextureView,
              _device: &wgpu::Device,
              queue: &wgpu::Queue,
              encoder: &mut wgpu::CommandEncoder)
    {
        self.light_binding.update(queue, &mut self.light);
        self.model_binding.update(queue, &mut self.model);
        self.camera_binding.update(queue, &mut self.camera);

        encoder.push_debug_group("ray cast render pass");
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
                            a: 0.0,
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