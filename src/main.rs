#![allow(dead_code)]

use std::borrow::Cow;
use std::io::{ErrorKind};
use std::{mem};
use std::cmp::max;
use std::f64::consts;
use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;
use crate::dataset::tag::Tag;
use crate::dicom_constants::tags::{PIXEL_DATA, STUDY_DATE, STUDY_INSTANCE_UID};
use crate::dicom_file_parser::dicom_file_parser::DicomFileParser;

use winit::{
    event::*,
};
use winit::keyboard::{KeyCode, PhysicalKey};
use crate::rendering::data_dimensions::DataDimensions;
use crate::rendering::utils::{Example, run};

mod data_reader;
mod dicom_file_parser;
mod dicom_constants;
mod dataset;
mod value_representations;
mod utils;
mod rendering;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Vertex {
    pub pos: [f32; 4],
    pub tex_coord: [f32; 2],
}

fn vertex(pos: [i8; 3], tc: [i8; 2]) -> Vertex {
    Vertex {
        pos: [pos[0] as f32, pos[1] as f32, pos[2] as f32, 1.0],
        tex_coord: [tc[0] as f32, tc[1] as f32],
    }
}

fn create_vertices(normalized_dims: [f32; 3]) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertex_data = [
        // top (0, 0, 1)
        vertex([-1, -1, 1], [0, 0]),
        vertex([1, -1, 1], [1, 0]),
        vertex([1, 1, 1], [1, 1]),
        vertex([-1, 1, 1], [0, 1]),
        // bottom (0, 0, -1)
        vertex([-1, 1, -1], [1, 0]),
        vertex([1, 1, -1], [0, 0]),
        vertex([1, -1, -1], [0, 1]),
        vertex([-1, -1, -1], [1, 1]),
        // right (1, 0, 0)
        vertex([1, -1, -1], [0, 0]),
        vertex([1, 1, -1], [1, 0]),
        vertex([1, 1, 1], [1, 1]),
        vertex([1, -1, 1], [0, 1]),
        // left (-1, 0, 0)
        vertex([-1, -1, 1], [1, 0]),
        vertex([-1, 1, 1], [0, 0]),
        vertex([-1, 1, -1], [0, 1]),
        vertex([-1, -1, -1], [1, 1]),
        // front (0, 1, 0)
        vertex([1, 1, -1], [1, 0]),
        vertex([-1, 1, -1], [0, 0]),
        vertex([-1, 1, 1], [0, 1]),
        vertex([1, 1, 1], [1, 1]),
        // back (0, -1, 0)
        vertex([1, -1, 1], [0, 0]),
        vertex([-1, -1, 1], [1, 0]),
        vertex([-1, -1, -1], [1, 1]),
        vertex([1, -1, -1], [0, 1]),
    ];

    vertex_data.
        iter_mut().
        for_each(|v| {
            v.pos[0] *= normalized_dims[0];
            v.pos[1] *= normalized_dims[1];
            v.pos[2] *= normalized_dims[2];
    });

    let index_data: &[u16] = &[
        0, 1, 2, 2, 3, 0, // top
        4, 5, 6, 6, 7, 4, // bottom
        8, 9, 10, 10, 11, 8, // right
        12, 13, 14, 14, 15, 12, // left
        16, 17, 18, 18, 19, 16, // front
        20, 21, 22, 22, 23, 20, // back
    ];

    (vertex_data.to_vec(), index_data.to_vec())
}

fn create_texels(size: usize) -> Vec<u8> {
    (0..size * size)
        .map(|id| {
            // get high five for recognizing this ;)
            let cx = 3.0 * (id % size) as f32 / (size - 1) as f32 - 2.0;
            let cy = 2.0 * (id / size) as f32 / (size - 1) as f32 - 1.0;
            let (mut x, mut y, mut count) = (cx, cy, 0);
            while count < 0xFF && x * x + y * y < 4.0 {
                let old_x = x;
                x = x * x - y * y + cx;
                y = 2.0 * old_x * y + cy;
                count += 1;
            }
            count
        })
        .collect()
}

pub struct Projection {
    pub aspect_ratio: f32,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

//https://www.3dgep.com/understanding-the-view-matrix/#The_View_Matrix
pub struct View {
    pub eye: glam::Vec3,
    pub target: glam::Vec3,
    pub up: glam::Vec3,
}

pub struct Model {
    pub rotation: glam::Quat,
    pub scale: glam::Vec3,
    pub translation: glam::Vec3,
}

pub struct ModelViewProjection {
    pub dicom_rotation: glam::Quat,
    pub model: Model,
    pub view: View,
    pub projection: Projection
}

struct Renderer {
    model_view_projection: ModelViewProjection,
    data_dims: DataDimensions,

    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
    index_count: usize,
    bind_group: wgpu::BindGroup,
    uniform_buf: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
    pipeline_wire: Option<wgpu::RenderPipeline>,
}

impl Renderer {
    fn generate_projection_matrix(projection: &Projection) -> glam::Mat4 {
        glam::Mat4::perspective_rh(projection.fov,
                                   projection.aspect_ratio,
                                   projection.near,
                                   projection.far)
    }

    fn generate_view_matrix(view: &View) -> glam::Mat4 {
        glam::Mat4::look_at_rh(view.eye,
                               view.target,
                               view.up)
    }

    fn generate_model_matrix(model: &Model) -> glam::Mat4 {
        glam::Mat4::from_scale_rotation_translation(model.scale,
                                                    model.rotation,
                                                    model.translation)
    }

    fn generate_matrix(patient_rotation: &glam::Quat, model: &Model, view: &View, projection: &Projection) -> glam::Mat4 {
        let projection = Self::generate_projection_matrix(&projection);
        let view = Self::generate_view_matrix(&view);
        let model = Self::generate_model_matrix(&model);
        let rotation = glam::Mat4::from_quat(patient_rotation.clone());

        rotation * projection * view * model
    }

    fn generate_mvp_matrix(&self) -> glam::Mat4 {
        Self::generate_matrix(&self.model_view_projection.dicom_rotation,
                              &self.model_view_projection.model,
                              &self.model_view_projection.view,
                              &self.model_view_projection.projection)
    }
}

impl Example for Renderer {
    fn optional_features() -> wgpu::Features {
        wgpu::Features::POLYGON_MODE_LINE
    }

    fn init(
        config: &wgpu::SurfaceConfiguration,
        _adapter: &wgpu::Adapter,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Self {
        // Create the data dimensions
        let data_dims = DataDimensions::builder()
            .width(512)
            .height(512)
            .depth(230)
            .pixel_spacing((0.8, 0.8))
            .distance_between_slices(1.0)
            .build();

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

        // Create pipeline layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(64),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        sample_type: wgpu::TextureSampleType::Uint,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create the texture
        let size = 256u32;
        let texels = create_texels(size as usize);
        let texture_extent = wgpu::Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: texture_extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Uint,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        queue.write_texture(
            texture.as_image_copy(),
            &texels,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(size),
                rows_per_image: None,
            },
            texture_extent,
        );

        let model_view_projection = ModelViewProjection {
            dicom_rotation: glam::Quat::IDENTITY,
            model: Model {
                rotation: glam::Quat::IDENTITY,
                scale: glam::Vec3::ONE,
                translation: glam::Vec3::ZERO,
            },
            view: View {
                eye: glam::Vec3::new(0.0, -5.0, 0.0),
                target: glam::Vec3::ZERO,
                up: glam::Vec3::Z,
            },
            projection: Projection {
                aspect_ratio: config.width as f32 / config.height as f32,
                fov: consts::FRAC_PI_4 as f32,
                near: 1.0,
                far: 10.0,
            }
        };

        // Create other resources
        let mx_total = Self::generate_matrix(&model_view_projection.dicom_rotation,
                                                   &model_view_projection.model,
                                                   &model_view_projection.view,
                                                   &model_view_projection.projection);

        let mx_ref: &[f32; 16] = mx_total.as_ref();
        let uniform_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(mx_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
            ],
            label: None,
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("rendering/shaders/shader.wgsl"))),
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
                    format: wgpu::VertexFormat::Float32x2,
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
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let pipeline_wire = if device
            .features()
            .contains(wgpu::Features::POLYGON_MODE_POINT)
        {
            let pipeline_wire = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &vertex_buffers,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_wire",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.view_formats[0],
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent {
                                operation: wgpu::BlendOperation::Add,
                                src_factor: wgpu::BlendFactor::SrcAlpha,
                                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            },
                            alpha: wgpu::BlendComponent::REPLACE,
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Line,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            });
            Some(pipeline_wire)
        } else {
            None
        };

        Renderer {
            model_view_projection,
            data_dims,
            vertex_buf,
            index_buf,
            index_count: index_data.len(),
            bind_group,
            uniform_buf,
            pipeline,
            pipeline_wire,
        }
    }

    fn resize(
        &mut self,
        config: &wgpu::SurfaceConfiguration,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        self.model_view_projection.projection.aspect_ratio = config.width as f32 / config.height as f32;

        let mvp = self.generate_mvp_matrix();
        queue.write_buffer(&self.uniform_buf, 0, bytemuck::cast_slice(mvp.as_ref()));
    }

    fn update(&mut self, _event: winit::event::WindowEvent) {
        //empty
    }

    fn render(&mut self, view: &wgpu::TextureView, device: &wgpu::Device, queue: &wgpu::Queue) {
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            rpass.push_debug_group("Prepare data for draw.");
            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.set_index_buffer(self.index_buf.slice(..), wgpu::IndexFormat::Uint16);
            rpass.set_vertex_buffer(0, self.vertex_buf.slice(..));
            rpass.pop_debug_group();
            rpass.insert_debug_marker("Draw!");
            rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);
            if let Some(ref pipe) = self.pipeline_wire {
                rpass.set_pipeline(pipe);
                rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);
            }
        }

        queue.submit(Some(encoder.finish()));
    }

    fn update_zoom(&mut self, zoom_delta: f32, queue: &wgpu::Queue) {
        const ZOOM_SPEED: f32 = 0.08;

        let zoom_delta = -zoom_delta * ZOOM_SPEED;
        self.model_view_projection.projection.fov += zoom_delta;
        self.model_view_projection.projection.fov = self.model_view_projection.projection.fov.min(1.0).max(0.25);

        println!("Zoom delta: {}", self.model_view_projection.projection.fov);

        let mvp = self.generate_mvp_matrix();
        queue.write_buffer(&self.uniform_buf, 0, bytemuck::cast_slice(mvp.as_ref()));
    }

    fn rotate(&mut self, dx: f32, dy: f32, queue: &wgpu::Queue) {
        let sensitivity = 0.005; // Adjust this value to your liking

        let dx = -dx * sensitivity;
        let dy = dy * sensitivity;

        let right = self.model_view_projection.view.eye.cross(self.model_view_projection.view.up).normalize();
        let up = right.cross(self.model_view_projection.view.eye).normalize();

        self.model_view_projection.model.rotation =
            glam::Quat::from_axis_angle(right, dy) *
            glam::Quat::from_axis_angle(up, dx) *
            self.model_view_projection.model.rotation;

        let mvp = self.generate_mvp_matrix();
        queue.write_buffer(&self.uniform_buf, 0, bytemuck::cast_slice(mvp.as_ref()));
    }
}

//
// const NUM_PARTICLES: u32 = 3;
//
// const PARTICLES_PER_GROUP: u32 = 64;
//
// struct Renderer {
//     particle_bind_groups: Vec<wgpu::BindGroup>,
//     particle_buffers: Vec<wgpu::Buffer>,
//     vertices_buffer: wgpu::Buffer,
//     compute_pipeline: wgpu::ComputePipeline,
//     render_pipeline: wgpu::RenderPipeline,
//     work_group_count: u32,
//     frame_num: usize,
// }
//
// impl Example for Renderer {
//     fn required_downlevel_capabilities() -> wgpu::DownlevelCapabilities {
//         wgpu::DownlevelCapabilities {
//             flags: wgpu::DownlevelFlags::COMPUTE_SHADERS,
//             ..Default::default()
//         }
//     }
//
//     fn required_limits() -> wgpu::Limits {
//         wgpu::Limits::downlevel_defaults()
//     }
//
//     /// constructs initial instance of Example struct
//     fn init(
//         config: &wgpu::SurfaceConfiguration,
//         _adapter: &wgpu::Adapter,
//         device: &wgpu::Device,
//         _queue: &wgpu::Queue,
//     ) -> Self {
//         let compute_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
//             label: None,
//             source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("rendering/shaders/compute.wgsl"))),
//         });
//
//         let draw_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
//             label: None,
//             source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("rendering/shaders/draw.wgsl"))),
//         });
//
//         // buffer for simulation parameters uniform
//
//         let sim_param_data = [
//             0.04f32, // deltaT
//             0.1,     // rule1Distance
//             0.025,   // rule2Distance
//             0.025,   // rule3Distance
//             0.02,    // rule1Scale
//             0.05,    // rule2Scale
//             0.005,   // rule3Scale
//         ].to_vec();
//
//         let sim_param_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//             label: Some("Simulation Parameter Buffer"),
//             contents: bytemuck::cast_slice(&sim_param_data),
//             usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
//         });
//
//         // create compute bind layout group and compute pipeline layout
//         let compute_bind_group_layout =
//             device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
//                 entries: &[
//                     wgpu::BindGroupLayoutEntry {
//                         binding: 0,
//                         visibility: wgpu::ShaderStages::COMPUTE,
//                         ty: wgpu::BindingType::Buffer {
//                             ty: wgpu::BufferBindingType::Uniform,
//                             has_dynamic_offset: false,
//                             min_binding_size: wgpu::BufferSize::new(
//                                 (sim_param_data.len() * mem::size_of::<f32>()) as _,
//                             ),
//                         },
//                         count: None,
//                     },
//                     wgpu::BindGroupLayoutEntry {
//                         binding: 1,
//                         visibility: wgpu::ShaderStages::COMPUTE,
//                         ty: wgpu::BindingType::Buffer {
//                             ty: wgpu::BufferBindingType::Storage { read_only: true },
//                             has_dynamic_offset: false,
//                             min_binding_size: wgpu::BufferSize::new((NUM_PARTICLES * 16) as _),
//                         },
//                         count: None,
//                     },
//                     wgpu::BindGroupLayoutEntry {
//                         binding: 2,
//                         visibility: wgpu::ShaderStages::COMPUTE,
//                         ty: wgpu::BindingType::Buffer {
//                             ty: wgpu::BufferBindingType::Storage { read_only: false },
//                             has_dynamic_offset: false,
//                             min_binding_size: wgpu::BufferSize::new((NUM_PARTICLES * 16) as _),
//                         },
//                         count: None,
//                     },
//                 ],
//                 label: None,
//             });
//
//         let compute_pipeline_layout =
//             device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//                 label: Some("compute"),
//                 bind_group_layouts: &[&compute_bind_group_layout],
//                 push_constant_ranges: &[],
//             });
//
//         // create render pipeline with empty bind group layout
//
//         let render_pipeline_layout =
//             device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//                 label: Some("render"),
//                 bind_group_layouts: &[],
//                 push_constant_ranges: &[],
//             });
//
//         let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
//             label: None,
//             layout: Some(&render_pipeline_layout),
//             vertex: wgpu::VertexState {
//                 module: &draw_shader,
//                 entry_point: "main_vs",
//                 buffers: &[
//                     wgpu::VertexBufferLayout {
//                         array_stride: 4 * 4,
//                         step_mode: wgpu::VertexStepMode::Instance,
//                         attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2],
//                     },
//                     wgpu::VertexBufferLayout {
//                         array_stride: 2 * 4,
//                         step_mode: wgpu::VertexStepMode::Vertex,
//                         attributes: &wgpu::vertex_attr_array![2 => Float32x2],
//                     },
//                 ],
//             },
//             fragment: Some(wgpu::FragmentState {
//                 module: &draw_shader,
//                 entry_point: "main_fs",
//                 targets: &[Some(config.view_formats[0].into())],
//             }),
//             primitive: wgpu::PrimitiveState::default(),
//             depth_stencil: None,
//             multisample: wgpu::MultisampleState::default(),
//             multiview: None,
//         });
//
//         // create compute pipeline
//
//         let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
//             label: Some("Compute pipeline"),
//             layout: Some(&compute_pipeline_layout),
//             module: &compute_shader,
//             entry_point: "main",
//         });
//
//         // buffer for the three 2d triangle vertices of each instance
//
//         let vertex_buffer_data = [-0.01f32, -0.02, 0.01, -0.02, 0.00, 0.02];
//         let vertices_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//             label: Some("Vertex Buffer"),
//             contents: bytemuck::bytes_of(&vertex_buffer_data),
//             usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
//         });
//
//         // buffer for all particles data of type [(posx,posy,velx,vely),...]
//
//         let mut initial_particle_data = vec![0.0f32; (4 * NUM_PARTICLES) as usize];
//         let mut rng = WyRand::new_seed(42);
//         let mut unif = || rng.generate::<f32>() * 2f32 - 1f32; // Generate a num (-1, 1)
//         for particle_instance_chunk in initial_particle_data.chunks_mut(4) {
//             particle_instance_chunk[0] = unif(); // posx
//             particle_instance_chunk[1] = unif(); // posy
//             particle_instance_chunk[2] = unif() * 0.1; // velx
//             particle_instance_chunk[3] = unif() * 0.1; // vely
//         }
//
//         // creates two buffers of particle data each of size NUM_PARTICLES
//         // the two buffers alternate as dst and src for each frame
//
//         let mut particle_buffers = Vec::<wgpu::Buffer>::new();
//         let mut particle_bind_groups = Vec::<wgpu::BindGroup>::new();
//         for i in 0..2 {
//             particle_buffers.push(
//                 device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//                     label: Some(&format!("Particle Buffer {i}")),
//                     contents: bytemuck::cast_slice(&initial_particle_data),
//                     usage: wgpu::BufferUsages::VERTEX
//                         | wgpu::BufferUsages::STORAGE
//                         | wgpu::BufferUsages::COPY_DST,
//                 }),
//             );
//         }
//
//         // create two bind groups, one for each buffer as the src
//         // where the alternate buffer is used as the dst
//
//         for i in 0..2 {
//             particle_bind_groups.push(device.create_bind_group(&wgpu::BindGroupDescriptor {
//                 layout: &compute_bind_group_layout,
//                 entries: &[
//                     wgpu::BindGroupEntry {
//                         binding: 0,
//                         resource: sim_param_buffer.as_entire_binding(),
//                     },
//                     wgpu::BindGroupEntry {
//                         binding: 1,
//                         resource: particle_buffers[i].as_entire_binding(),
//                     },
//                     wgpu::BindGroupEntry {
//                         binding: 2,
//                         resource: particle_buffers[(i + 1) % 2].as_entire_binding(), // bind to opposite buffer
//                     },
//                 ],
//                 label: None,
//             }));
//         }
//
//         // calculates number of work groups from PARTICLES_PER_GROUP constant
//         let work_group_count =
//             ((NUM_PARTICLES as f32) / (PARTICLES_PER_GROUP as f32)).ceil() as u32;
//
//         // returns Example struct and No encoder commands
//
//         Renderer {
//             particle_bind_groups,
//             particle_buffers,
//             vertices_buffer,
//             compute_pipeline,
//             render_pipeline,
//             work_group_count,
//             frame_num: 0,
//         }
//     }
//
//     /// resize is called on WindowEvent::Resized events
//     fn resize(
//         &mut self,
//         _sc_desc: &wgpu::SurfaceConfiguration,
//         _device: &wgpu::Device,
//         _queue: &wgpu::Queue,
//     ) {
//         //empty
//     }
//
//     /// update is called for any WindowEvent not handled by the framework
//     fn update(&mut self, _event: winit::event::WindowEvent) {
//         //empty
//     }
//
//     /// render is called each frame, dispatching compute groups proportional
//     ///   a TriangleList draw call for all NUM_PARTICLES at 3 vertices each
//     fn render(&mut self, view: &wgpu::TextureView, device: &wgpu::Device, queue: &wgpu::Queue) {
//         let color_attachments = [Some(wgpu::RenderPassColorAttachment {
//             view,
//             resolve_target: None,
//             ops: wgpu::Operations {
//                 load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
//                 store: wgpu::StoreOp::Store,
//             },
//         })];
//
//         let render_pass_descriptor = wgpu::RenderPassDescriptor {
//             label: None,
//             color_attachments: &color_attachments,
//             depth_stencil_attachment: None,
//             timestamp_writes: None,
//             occlusion_query_set: None,
//         };
//
//         // get command encoder
//         let mut command_encoder =
//             device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
//
//         command_encoder.push_debug_group("compute boid movement");
//         {
//             // compute pass
//             let mut cpass = command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
//                 label: None,
//                 timestamp_writes: None,
//             });
//             cpass.set_pipeline(&self.compute_pipeline);
//             cpass.set_bind_group(0, &self.particle_bind_groups[self.frame_num % 2], &[]);
//             cpass.dispatch_workgroups(self.work_group_count, 1, 1);
//         }
//         command_encoder.pop_debug_group();
//
//         command_encoder.push_debug_group("render boids");
//         {
//             // render pass
//             let mut rpass = command_encoder.begin_render_pass(&render_pass_descriptor);
//             rpass.set_pipeline(&self.render_pipeline);
//             // render dst particles
//             rpass.set_vertex_buffer(0, self.particle_buffers[(self.frame_num + 1) % 2].slice(..));
//             // the three instance-local vertices
//             rpass.set_vertex_buffer(1, self.vertices_buffer.slice(..));
//             rpass.draw(0..3, 0..NUM_PARTICLES);
//         }
//         command_encoder.pop_debug_group();
//
//         // update frame count
//         self.frame_num += 1;
//
//         // done
//         queue.submit(Some(command_encoder.finish()));
//     }
// }
//
//
//
// const TEXTURE_DIMS: (usize, usize) = (512, 512);

// pub fn output_image_native(image_data: Vec<u8>, texture_dims: (usize, usize), path: String) {
//     let mut png_data = Vec::<u8>::with_capacity(image_data.len());
//     let mut encoder = png::Encoder::new(
//         std::io::Cursor::new(&mut png_data),
//         texture_dims.0 as u32,
//         texture_dims.1 as u32,
//     );
//     encoder.set_color(png::ColorType::Rgba);
//     let mut png_writer = encoder.write_header().unwrap();
//     png_writer.write_image_data(&image_data[..]).unwrap();
//     png_writer.finish().unwrap();
//     log::info!("PNG file encoded in memory.");
//
//     let mut file = std::fs::File::create(&path).unwrap();
//     file.write_all(&png_data[..]).unwrap();
//     log::info!("PNG file written to disc as \"{}\".", path);
// }

// async fn run(window: Window, _path: Option<String>) {
//     let instance = wgpu::Instance::default();
//
//     let adapter = instance.request_adapter(
//         &wgpu::RequestAdapterOptions {
//             power_preference: wgpu::PowerPreference::HighPerformance,
//             compatible_surface: Some(&surface),
//             force_fallback_adapter: false,
//         })
//         .await
//         .unwrap();
//
//     let (device, queue) = adapter
//         .request_device(
//             &wgpu::DeviceDescriptor {
//                 label: None,
//                 required_features: wgpu::Features::empty(),
//                 required_limits: wgpu::Limits::default(),
//             },
//             None,
//         )
//         .await
//         .unwrap();
//
//
//     let size = window.inner_size();
//     let surface = unsafe { instance.create_surface(&window) }.unwrap();
//     let surface_caps = surface.get_capabilities(&adapter);
//
//     let surface_format = surface_caps.formats.iter()
//         .copied()
//         .filter(|f| f.is_srgb())
//         .next()
//         .unwrap_or(surface_caps.formats[0]);
//
//     surface.configure(&device, &wgpu::SurfaceConfiguration {
//         usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
//         format: surface_format,
//         width: size.width,
//         height: size.height,
//         present_mode: surface_caps.present_modes[0],
//         desired_maximum_frame_latency: 2,
//         alpha_mode: surface_caps.alpha_modes[0],
//         view_formats: surface_caps.formats,
//     });
//
//     let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
//         label: None,
//         source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("shader.wgsl"))),
//     });
//
//     let storage_texture = device.create_texture(&wgpu::TextureDescriptor {
//         label: None,
//         size: wgpu::Extent3d {
//             width: TEXTURE_DIMS.0 as u32,
//             height: TEXTURE_DIMS.1 as u32,
//             depth_or_array_layers: 1,
//         },
//         mip_level_count: 1,
//         sample_count: 1,
//         dimension: wgpu::TextureDimension::D2,
//         format: wgpu::TextureFormat::Rgba8Unorm,
//         usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
//         view_formats: &[],
//     });
//
//     let mut texture_data = vec![0u8; TEXTURE_DIMS.0 * TEXTURE_DIMS.1 * 4];
//
//     let storage_texture_view = storage_texture.create_view(&wgpu::TextureViewDescriptor::default());
//     let output_staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
//         label: None,
//         size: std::mem::size_of_val(&texture_data[..]) as u64,
//         usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
//         mapped_at_creation: false,
//     });
//
//     let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
//         label: None,
//         entries: &[wgpu::BindGroupLayoutEntry {
//             binding: 0,
//             visibility: wgpu::ShaderStages::COMPUTE,
//             ty: wgpu::BindingType::StorageTexture {
//                 access: wgpu::StorageTextureAccess::WriteOnly,
//                 format: wgpu::TextureFormat::Rgba8Unorm,
//                 view_dimension: wgpu::TextureViewDimension::D2,
//             },
//             count: None,
//         }],
//     });
//
//     let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
//         label: None,
//         layout: &bind_group_layout,
//         entries: &[wgpu::BindGroupEntry {
//             binding: 0,
//             resource: wgpu::BindingResource::TextureView(&storage_texture_view),
//         }],
//     });
//
//     let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//         label: None,
//         bind_group_layouts: &[&bind_group_layout],
//         push_constant_ranges: &[],
//     });
//
//     let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
//         label: None,
//         layout: Some(&pipeline_layout),
//         module: &shader,
//         entry_point: "main",
//     });
//
//     log::info!("Wgpu context set up.");
//     //----------------------------------------
//
//     let mut command_encoder =
//         device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
//     {
//         let mut compute_pass = command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
//             label: None,
//             timestamp_writes: None,
//         });
//         compute_pass.set_bind_group(0, &bind_group, &[]);
//         compute_pass.set_pipeline(&pipeline);
//         compute_pass.dispatch_workgroups(TEXTURE_DIMS.0 as u32, TEXTURE_DIMS.1 as u32, 1);
//     }
//
//     command_encoder.copy_texture_to_buffer(
//         wgpu::ImageCopyTexture {
//             texture: &storage_texture,
//             mip_level: 0,
//             origin: wgpu::Origin3d::ZERO,
//             aspect: wgpu::TextureAspect::All,
//         },
//         wgpu::ImageCopyBuffer {
//             buffer: &output_staging_buffer,
//             layout: wgpu::ImageDataLayout {
//                 offset: 0,
//                 // This needs to be padded to 256.
//                 bytes_per_row: Some((TEXTURE_DIMS.0 * 4) as u32),
//                 rows_per_image: Some(TEXTURE_DIMS.1 as u32),
//             },
//         },
//         wgpu::Extent3d {
//             width: TEXTURE_DIMS.0 as u32,
//             height: TEXTURE_DIMS.1 as u32,
//             depth_or_array_layers: 1,
//         },
//     );
//     queue.submit(Some(command_encoder.finish()));
//
//     let buffer_slice = output_staging_buffer.slice(..);
//     let (sender, receiver) = flume::bounded(1);
//     buffer_slice.map_async(wgpu::MapMode::Read, move |r| sender.send(r).unwrap());
//     device.poll(wgpu::Maintain::wait()).panic_on_timeout();
//     receiver.recv_async().await.unwrap().unwrap();
//     log::info!("Output buffer mapped");
//     {
//         let view = buffer_slice.get_mapped_range();
//         texture_data.copy_from_slice(&view[..]);
//     }
//     log::info!("GPU data copied to local.");
//     output_staging_buffer.unmap();
//
//     output_image_native(texture_data.to_vec(), TEXTURE_DIMS, _path.unwrap());
//     log::info!("Done.")
// }

fn is_close_requested(event: &Event<()>) -> bool {
    if let Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
    } = event {
        true
    } else {
        false
    }
}

fn is_escape_pressed(event: &Event<()>) -> bool {
    if let Event::WindowEvent {
        event: WindowEvent::KeyboardInput {
            event: KeyEvent {
                state: ElementState::Pressed,
                physical_key: PhysicalKey::Code(KeyCode::Escape),
                ..
            },
            ..
        },
        ..
    } = event {
        true
    } else {
        false
    }
}

fn is_close_window_requested(event: &Event<()>) -> bool {
    is_escape_pressed(event) || is_close_requested(event)
}

fn main()  -> std::io::Result<()>
{
    let path = "C:/Users/medapp/Desktop/CT/0015.dcm";

    pub const IMAGE_POSITION: Tag = Tag { group: 0x0020, element: 0x0032 };
    pub const IMAGE_ORIENTATION: Tag = Tag { group: 0x0020, element: 0x0037 };
    pub const SAMPLES_PER_PIXEL: Tag = Tag { group: 0x0028, element: 0x0002 };
    pub const PHOTOMETRIC_INTERPRETATION: Tag = Tag { group: 0x0028, element: 0x0004 };
    pub const ROWS: Tag = Tag { group: 0x0028, element: 0x0010 };
    pub const COLUMNS: Tag = Tag { group: 0x0028, element: 0x0011 };
    pub const PIXEL_SPACING: Tag = Tag { group: 0x0028, element: 0x0030 };
    pub const BITS_ALLOCATED: Tag = Tag { group: 0x0028, element: 0x0100 };
    pub const BITS_STORED: Tag = Tag { group: 0x0028, element: 0x0101 };
    pub const HIGH_BIT: Tag = Tag { group: 0x0028, element: 0x0102 };
    pub const PIXEL_REPRESENTATION: Tag = Tag { group: 0x0028, element: 0x0103 };
    pub const WINDOW_CENTER: Tag = Tag { group: 0x0028, element: 0x1050 };
    pub const WINDOW_WIDTH: Tag = Tag { group: 0x0028, element: 0x1051 };
    pub const RESCALE_INTERCEPT: Tag = Tag { group: 0x0028, element: 0x1052 };
    pub const RESCALE_SLOPE: Tag = Tag { group: 0x0028, element: 0x1053 };

    let tags_to_read = [
        STUDY_DATE,
        STUDY_INSTANCE_UID,
        IMAGE_POSITION,
        IMAGE_ORIENTATION,
        SAMPLES_PER_PIXEL,
        PHOTOMETRIC_INTERPRETATION,
        ROWS,
        COLUMNS,
        PIXEL_SPACING,
        BITS_ALLOCATED,
        BITS_STORED,
        HIGH_BIT,
        PIXEL_REPRESENTATION,
        WINDOW_CENTER,
        WINDOW_WIDTH,
        RESCALE_INTERCEPT,
        RESCALE_SLOPE,
        PIXEL_DATA].as_ref();

    let dicom_data_elems = DicomFileParser::new()
                 .file_path(path)
                 .read_tags(tags_to_read)
                 .with_lazy_read_element(Some(10))
                 .parse();

    if let Err(e) = dicom_data_elems {
        println!("Error: {}", e);
        return Err(std::io::Error::new(ErrorKind::Other, "An error occurred"));
    }

    let data_elems = dicom_data_elems.unwrap();
    for elem in data_elems {
        println!("{:?}", elem);
    }

    run::<Renderer>("Dicom Viewer");

    Ok(())
}