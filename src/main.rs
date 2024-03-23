#![allow(dead_code)]

use std::io::{Error as OtherError, ErrorKind, Write};
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

const TEXTURE_DIMS: (usize, usize) = (512, 512);

pub fn output_image_native(image_data: Vec<u8>, texture_dims: (usize, usize), path: String) {
    let mut png_data = Vec::<u8>::with_capacity(image_data.len());
    let mut encoder = png::Encoder::new(
        std::io::Cursor::new(&mut png_data),
        texture_dims.0 as u32,
        texture_dims.1 as u32,
    );
    encoder.set_color(png::ColorType::Rgba);
    let mut png_writer = encoder.write_header().unwrap();
    png_writer.write_image_data(&image_data[..]).unwrap();
    png_writer.finish().unwrap();
    log::info!("PNG file encoded in memory.");

    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(&png_data[..]).unwrap();
    log::info!("PNG file written to disc as \"{}\".", path);
}

async fn run(window: Window, _path: Option<String>) {
    let instance = wgpu::Instance::default();

    let surface = unsafe { instance.create_surface(&window) }.unwrap();

    let adapter = instance.request_adapter(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
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
                required_limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .unwrap();

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("shader.wgsl"))),
    });

    let storage_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size: wgpu::Extent3d {
            width: TEXTURE_DIMS.0 as u32,
            height: TEXTURE_DIMS.1 as u32,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let mut texture_data = vec![0u8; TEXTURE_DIMS.0 * TEXTURE_DIMS.1 * 4];
    let storage_texture_view = storage_texture.create_view(&wgpu::TextureViewDescriptor::default());
    let output_staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: std::mem::size_of_val(&texture_data[..]) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::StorageTexture {
                access: wgpu::StorageTextureAccess::WriteOnly,
                format: wgpu::TextureFormat::Rgba8Unorm,
                view_dimension: wgpu::TextureViewDimension::D2,
            },
            count: None,
        }],
    });
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::TextureView(&storage_texture_view),
        }],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: "main",
    });

    log::info!("Wgpu context set up.");
    //----------------------------------------

    let mut command_encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut compute_pass = command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: None,
            timestamp_writes: None,
        });
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.set_pipeline(&pipeline);
        compute_pass.dispatch_workgroups(TEXTURE_DIMS.0 as u32, TEXTURE_DIMS.1 as u32, 1);
    }

    command_encoder.copy_texture_to_buffer(
        wgpu::ImageCopyTexture {
            texture: &storage_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::ImageCopyBuffer {
            buffer: &output_staging_buffer,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                // This needs to be padded to 256.
                bytes_per_row: Some((TEXTURE_DIMS.0 * 4) as u32),
                rows_per_image: Some(TEXTURE_DIMS.1 as u32),
            },
        },
        wgpu::Extent3d {
            width: TEXTURE_DIMS.0 as u32,
            height: TEXTURE_DIMS.1 as u32,
            depth_or_array_layers: 1,
        },
    );
    queue.submit(Some(command_encoder.finish()));

    let buffer_slice = output_staging_buffer.slice(..);
    let (sender, receiver) = flume::bounded(1);
    buffer_slice.map_async(wgpu::MapMode::Read, move |r| sender.send(r).unwrap());
    device.poll(wgpu::Maintain::wait()).panic_on_timeout();
    receiver.recv_async().await.unwrap().unwrap();
    log::info!("Output buffer mapped");
    {
        let view = buffer_slice.get_mapped_range();
        texture_data.copy_from_slice(&view[..]);
    }
    log::info!("GPU data copied to local.");
    output_staging_buffer.unmap();

    output_image_native(texture_data.to_vec(), TEXTURE_DIMS, _path.unwrap());
    log::info!("Done.")
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

    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp_nanos()
        .init();

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    let _ = event_loop.run(move |event, elwt| {
        if is_escape_pressed(&event) || is_close_requested(&event) {
            println!("The escape key was pressed; stopping");
            elwt.exit();
        }
    });

    let path = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "please_don't_git_push_me.png".to_string());
    pollster::block_on(run(Some(path)));

    Ok(())
}

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