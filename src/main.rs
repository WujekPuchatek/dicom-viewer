#![allow(dead_code)]

use std::borrow::Cow;
use std::io::{Error as OtherError, ErrorKind, Write};
use std::{iter, mem};
use std::rc::Rc;
use wgpu::util::DeviceExt;
use crate::dataset::tag::Tag;
use crate::dicom_constants::tags::{PIXEL_DATA, SERIES_INSTANCE_UID, STUDY_DATE, STUDY_INSTANCE_UID};
use crate::dicom_file_parser::dicom_file_parser::DicomFileParser;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit::event::WindowEvent::KeyboardInput;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::Window;
use crate::rendering::utils::{Example, run};
use nanorand::{Rng, WyRand};

mod data_reader;
mod dicom_file_parser;
mod dicom_constants;
mod dataset;
mod value_representations;
mod utils;
mod rendering;

const NUM_PARTICLES: u32 = 3;

const PARTICLES_PER_GROUP: u32 = 64;

struct Renderer {
    particle_bind_groups: Vec<wgpu::BindGroup>,
    particle_buffers: Vec<wgpu::Buffer>,
    vertices_buffer: wgpu::Buffer,
    compute_pipeline: wgpu::ComputePipeline,
    render_pipeline: wgpu::RenderPipeline,
    work_group_count: u32,
    frame_num: usize,
}

impl Example for Renderer {
    fn required_downlevel_capabilities() -> wgpu::DownlevelCapabilities {
        wgpu::DownlevelCapabilities {
            flags: wgpu::DownlevelFlags::COMPUTE_SHADERS,
            ..Default::default()
        }
    }

    fn required_limits() -> wgpu::Limits {
        wgpu::Limits::downlevel_defaults()
    }

    /// constructs initial instance of Example struct
    fn init(
        config: &wgpu::SurfaceConfiguration,
        _adapter: &wgpu::Adapter,
        device: &wgpu::Device,
        _queue: &wgpu::Queue,
    ) -> Self {
        let compute_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("rendering/shaders/compute.wgsl"))),
        });

        let draw_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("rendering/shaders/draw.wgsl"))),
        });

        // buffer for simulation parameters uniform

        let sim_param_data = [
            0.04f32, // deltaT
            0.1,     // rule1Distance
            0.025,   // rule2Distance
            0.025,   // rule3Distance
            0.02,    // rule1Scale
            0.05,    // rule2Scale
            0.005,   // rule3Scale
        ].to_vec();

        let sim_param_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Simulation Parameter Buffer"),
            contents: bytemuck::cast_slice(&sim_param_data),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // create compute bind layout group and compute pipeline layout
        let compute_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: wgpu::BufferSize::new(
                                (sim_param_data.len() * mem::size_of::<f32>()) as _,
                            ),
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: wgpu::BufferSize::new((NUM_PARTICLES * 16) as _),
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: false },
                            has_dynamic_offset: false,
                            min_binding_size: wgpu::BufferSize::new((NUM_PARTICLES * 16) as _),
                        },
                        count: None,
                    },
                ],
                label: None,
            });

        let compute_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("compute"),
                bind_group_layouts: &[&compute_bind_group_layout],
                push_constant_ranges: &[],
            });

        // create render pipeline with empty bind group layout

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("render"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &draw_shader,
                entry_point: "main_vs",
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: 4 * 4,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: 2 * 4,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![2 => Float32x2],
                    },
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &draw_shader,
                entry_point: "main_fs",
                targets: &[Some(config.view_formats[0].into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        // create compute pipeline

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute pipeline"),
            layout: Some(&compute_pipeline_layout),
            module: &compute_shader,
            entry_point: "main",
        });

        // buffer for the three 2d triangle vertices of each instance

        let vertex_buffer_data = [-0.01f32, -0.02, 0.01, -0.02, 0.00, 0.02];
        let vertices_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::bytes_of(&vertex_buffer_data),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        // buffer for all particles data of type [(posx,posy,velx,vely),...]

        let mut initial_particle_data = vec![0.0f32; (4 * NUM_PARTICLES) as usize];
        let mut rng = WyRand::new_seed(42);
        let mut unif = || rng.generate::<f32>() * 2f32 - 1f32; // Generate a num (-1, 1)
        for particle_instance_chunk in initial_particle_data.chunks_mut(4) {
            particle_instance_chunk[0] = unif(); // posx
            particle_instance_chunk[1] = unif(); // posy
            particle_instance_chunk[2] = unif() * 0.1; // velx
            particle_instance_chunk[3] = unif() * 0.1; // vely
        }

        // creates two buffers of particle data each of size NUM_PARTICLES
        // the two buffers alternate as dst and src for each frame

        let mut particle_buffers = Vec::<wgpu::Buffer>::new();
        let mut particle_bind_groups = Vec::<wgpu::BindGroup>::new();
        for i in 0..2 {
            particle_buffers.push(
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("Particle Buffer {i}")),
                    contents: bytemuck::cast_slice(&initial_particle_data),
                    usage: wgpu::BufferUsages::VERTEX
                        | wgpu::BufferUsages::STORAGE
                        | wgpu::BufferUsages::COPY_DST,
                }),
            );
        }

        // create two bind groups, one for each buffer as the src
        // where the alternate buffer is used as the dst

        for i in 0..2 {
            particle_bind_groups.push(device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &compute_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: sim_param_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: particle_buffers[i].as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: particle_buffers[(i + 1) % 2].as_entire_binding(), // bind to opposite buffer
                    },
                ],
                label: None,
            }));
        }

        // calculates number of work groups from PARTICLES_PER_GROUP constant
        let work_group_count =
            ((NUM_PARTICLES as f32) / (PARTICLES_PER_GROUP as f32)).ceil() as u32;

        // returns Example struct and No encoder commands

        Renderer {
            particle_bind_groups,
            particle_buffers,
            vertices_buffer,
            compute_pipeline,
            render_pipeline,
            work_group_count,
            frame_num: 0,
        }
    }

    /// resize is called on WindowEvent::Resized events
    fn resize(
        &mut self,
        _sc_desc: &wgpu::SurfaceConfiguration,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
    ) {
        //empty
    }

    /// update is called for any WindowEvent not handled by the framework
    fn update(&mut self, _event: winit::event::WindowEvent) {
        //empty
    }

    /// render is called each frame, dispatching compute groups proportional
    ///   a TriangleList draw call for all NUM_PARTICLES at 3 vertices each
    fn render(&mut self, view: &wgpu::TextureView, device: &wgpu::Device, queue: &wgpu::Queue) {
        let color_attachments = [Some(wgpu::RenderPassColorAttachment {
            view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                store: wgpu::StoreOp::Store,
            },
        })];

        let render_pass_descriptor = wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &color_attachments,
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        };

        // get command encoder
        let mut command_encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        command_encoder.push_debug_group("compute boid movement");
        {
            // compute pass
            let mut cpass = command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });
            cpass.set_pipeline(&self.compute_pipeline);
            cpass.set_bind_group(0, &self.particle_bind_groups[self.frame_num % 2], &[]);
            cpass.dispatch_workgroups(self.work_group_count, 1, 1);
        }
        command_encoder.pop_debug_group();

        command_encoder.push_debug_group("render boids");
        {
            // render pass
            let mut rpass = command_encoder.begin_render_pass(&render_pass_descriptor);
            rpass.set_pipeline(&self.render_pipeline);
            // render dst particles
            rpass.set_vertex_buffer(0, self.particle_buffers[(self.frame_num + 1) % 2].slice(..));
            // the three instance-local vertices
            rpass.set_vertex_buffer(1, self.vertices_buffer.slice(..));
            rpass.draw(0..3, 0..NUM_PARTICLES);
        }
        command_encoder.pop_debug_group();

        // update frame count
        self.frame_num += 1;

        // done
        queue.submit(Some(command_encoder.finish()));
    }
}



const TEXTURE_DIMS: (usize, usize) = (512, 512);

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