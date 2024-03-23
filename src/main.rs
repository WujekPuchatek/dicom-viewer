#![allow(dead_code)]

use std::io::{Error as OtherError, ErrorKind, Write};
use std::iter;
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

mod data_reader;
mod dicom_file_parser;
mod dicom_constants;
mod dataset;
mod value_representations;
mod utils;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.0868241, 0.49240386, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // A
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // B
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // C
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // D
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // E
];

const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4, /* padding */ 0];

struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32
}

impl State<'_> {
    async fn new(window: Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: Default::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = INDICES.len() as u32;

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    #[allow(unused_variables)]
    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {}

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
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
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
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

pub async fn run() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp_nanos()
        .init();

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // State::new uses async code, so we're going to wait for it to finish
    let mut state = State::new(window).await;

    let _ = event_loop.run(move |event, elwt| {
        if is_close_window_requested(&event) {
            println!("The escape key was pressed; stopping");
            elwt.exit();
        }

        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::Resized(physical_size) => {
                    state.resize(physical_size);
                }
                WindowEvent::RedrawRequested => {
                    state.update();
                    match state.render() {
                        Ok(_) => {}
                        // Reconfigure the surface if it's lost or outdated
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            state.resize(state.size)
                        }
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),

                        Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                    }
                },
                _ => {}
                }
            }
            _ => {},
        _ => {}
        }
    });
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

    pollster::block_on(run());

    Ok(())
}