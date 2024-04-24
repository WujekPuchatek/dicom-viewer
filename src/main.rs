#![allow(dead_code)]
#![feature(test)]

use std::io::{ErrorKind, Read, Write};
use std::{io, mem};
use std::time::Instant;
use bytemuck::cast_slice;
use crate::dicom_constants::tags::*;
use crate::dicom_file_parser::dicom_file_parser::DicomFileParser;

use winit::{
    event::*,
};
use winit::keyboard::{KeyCode, PhysicalKey};
use crate::dicom_file::dicom_file::DicomFile;
use crate::examination::examination::Examination;
use crate::examinations::examinations::Examinations;
use crate::rendering::renderers::renderer::Renderer;
use crate::files_finder::files_finder::{FilesFinder, FindFiles};
use crate::rendering::compute_shaders::compute_normal_to_surface::ComputeNormalToSurface;
use crate::rendering::compute_shaders::compute_shader::ComputeShader;
use crate::rendering::compute_shaders::rescale_values::ComputeRescaleValues;
use crate::rendering::renderers::raycast_renderer::RayCastRenderer;
use crate::rendering::utils::{Example, run};

mod data_reader;
mod dicom_file_parser;
mod dicom_constants;
mod dataset;
mod value_representations;
mod utils;
mod rendering;
mod examination;
mod dicom_file;
mod information_object_definitions;
mod traits;
mod examinations;
mod files_finder;
mod pixel_data_processor;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

struct MainRenderer {
    texture: wgpu::Texture,
    texture_view : wgpu::TextureView,

    compute_normal_to_surface: ComputeNormalToSurface,
    values_rescaler : ComputeRescaleValues,
    raycast_renderer: RayCastRenderer,
}

impl Example for MainRenderer {
    fn optional_features() -> wgpu::Features {
        wgpu::Features::POLYGON_MODE_LINE
    }

    fn init(
        config: &wgpu::SurfaceConfiguration,
        adapter: &wgpu::Adapter,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        exam: &Examination
    ) -> Self {
        let data_dims = exam.get_dimensions();

        let texture_extent = wgpu::Extent3d {
            width: data_dims.width,
            height: data_dims.height,
            depth_or_array_layers: data_dims.depth,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: texture_extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D3,
            format: wgpu::TextureFormat::R32Float,
            usage: wgpu::TextureUsages::TEXTURE_BINDING |
                   wgpu::TextureUsages::STORAGE_BINDING |
                   wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let data = exam. get_image_data();
        queue.write_texture(
            texture.as_image_copy(),
            cast_slice(&data),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(data_dims.width * mem::size_of::<f32>() as u32),
                rows_per_image: Some(data_dims.height),

            },
            texture_extent,
        );

        let values_rescaler = ComputeRescaleValues::init(
            adapter,
            device,
            queue,
            exam,
            &texture_view
        );

        let compute_normal_to_surface = ComputeNormalToSurface::init(
            adapter,
            device,
            queue,
            &data_dims,
            &texture_view
        );

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Command Encoder for GPU compute"),
        });

        values_rescaler.step(device, queue, &mut encoder);
        compute_normal_to_surface.step(device, queue, &mut encoder);

        queue.submit(Some(encoder.finish()));

        let raycast_renderer = RayCastRenderer::init(
            config,
            adapter,
            device,
            queue,
            exam,
            &compute_normal_to_surface.get_normal_to_surface_view()
        );

        MainRenderer {
            texture,
            texture_view,
            compute_normal_to_surface,
            values_rescaler,
            raycast_renderer
        }
    }

    fn resize(
        &mut self,
        config: &wgpu::SurfaceConfiguration,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        self.raycast_renderer.resize(config, device, queue);
    }

    fn update(&mut self, _event: winit::event::WindowEvent) {
        //empty
    }

    fn render(&mut self, view: &wgpu::TextureView, device: &wgpu::Device, queue: &wgpu::Queue) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Command Encoder for GPU compute"),
        });

        self.raycast_renderer.render(view, device, queue, &mut encoder);

        queue.submit(Some(encoder.finish()));
    }

    fn update_zoom(&mut self, zoom_delta: f32, queue: &wgpu::Queue) {
        self.raycast_renderer.update_zoom(zoom_delta, queue);
    }

    fn rotate(&mut self, dx: f32, dy: f32, queue: &wgpu::Queue) {
        self.raycast_renderer.rotate(dx, dy, queue);
    }
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

fn is_close_window_requested(event: &Event<()>) -> bool {
    is_escape_pressed(event) || is_close_requested(event)
}

fn main()  -> std::io::Result<()>
{
    let exam_path = "C://Dane//OneDrive_2023-09-13//70 % 1.0  B30f";
    let files = FilesFinder::new().find_files(exam_path);
    let mut exams = Examinations::new();

    let start = Instant::now();

    for file in files {
        let tags_to_read = [
            MODALITY,
            STUDY_DATE,
            STUDY_INSTANCE_UID,
            SERIES_INSTANCE_UID,
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
            .file_path(file.as_str())
            .read_tags(tags_to_read)
            .with_lazy_read_element(Some(256))
            .parse();

        if let Err(e) = dicom_data_elems {
            println!("Error: {}", e);
            return Err(std::io::Error::new(ErrorKind::Other, "An error occurred"));
        }

        let data_elems = dicom_data_elems.unwrap();

        let factory = DicomFile::factory();
        let dicom_file = factory.create(file.as_str(), data_elems);

        if let Err(e) = dicom_file {
            let as_str = e.into_iter().fold(String::new(),
                                            |acc, e|
                                                acc + format!("{:?}", e).as_str() + "\n");

            println!("{}", as_str);
            return Err(std::io::Error::new(ErrorKind::Other, "An error occurred"));
        }

        let dicom_file = dicom_file.ok().unwrap();
        exams.add_dicom_file(dicom_file);
    }

    let exam = exams.get_examinations()[0];

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

    run::<MainRenderer>("Dicom Viewer", exam);

    Ok(())
}