use bytemuck::{Pod, Zeroable};
use glam::{Vec3};
use wgpu::util::DeviceExt;
use crate::utils::non_zero_sized::NonZeroSized;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct LightUniform {
    pub position: Vec3,
    spacing1: f32,

    pub ambient : Vec3,
    spacing2: f32,

    pub diffuse : Vec3,
    spacing3: f32,

    pub specular : Vec3,
    spacing4: f32,
}
unsafe impl Pod for LightUniform {}
unsafe impl Zeroable for LightUniform {}

impl Default for LightUniform {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            spacing1: 0.0,
            ambient: Vec3::ZERO,
            spacing2: 0.0,
            diffuse: Vec3::ZERO,
            spacing3: 0.0,
            specular: Vec3::ZERO,
            spacing4: 0.0,
        }
    }
}

impl LightUniform {
    pub fn new(position: Vec3, ambient: Vec3, diffuse: Vec3, specular: Vec3) -> Self {
        Self {
            position,
            ambient,
            diffuse,
            specular,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub position: Vec3,
    pub ambient : Vec3,
    pub diffuse : Vec3,
    pub specular : Vec3,

    updated: bool,
}

impl Light {
    pub fn new() -> Self {
        Self {
            position: Vec3::new(2.0, -5.0, 2.0),
            ambient: Vec3::new(0.2, 0.2, 0.2),
            diffuse: Vec3::new(0.5, 0.5, 0.5),
            specular: Vec3::new(1.0, 1.0, 1.0),
            updated: true,
        }
    }

    pub fn get_light(&self) -> LightUniform {
        LightUniform::new(self.position, self.ambient, self.diffuse, self.specular)
    }
}

pub struct LightBinding {
    pub buffer: wgpu::Buffer,
    binding_index: u32
}

impl LightBinding {
    pub fn bind_group_layout_entry(&self) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: self.binding_index,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: Some(LightUniform::SIZE),
            },
            count: None,
        }
    }

    pub fn bind_group_entry(&self) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding: self.binding_index,
            resource: self.buffer.as_entire_binding(),
        }
    }

    pub fn new(device: &wgpu::Device, binding_index: u32) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Light Buffer"),
            contents: bytemuck::bytes_of(&LightUniform::default()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        Self { buffer, binding_index }
    }

    pub fn update(&mut self, queue: &wgpu::Queue, light: &mut Light) {
        if light.updated {
            queue.write_buffer(
                &self.buffer,
                0,
                bytemuck::bytes_of(&light.get_light()),
            );
            light.updated = false;
        }
    }
}

