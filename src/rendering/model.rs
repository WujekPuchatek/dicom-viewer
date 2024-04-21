use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Quat, Vec3};
use wgpu::util::DeviceExt;
use crate::utils::non_zero_sized::NonZeroSized;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ModelUniform {
    pub model: Mat4,
    pub inv_model: Mat4,
}
unsafe impl Pod for ModelUniform {}
unsafe impl Zeroable for ModelUniform {}

impl Default for ModelUniform {
    fn default() -> Self {
        Self {
            model: Mat4::IDENTITY,
            inv_model: Mat4::IDENTITY,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Model {
    pub rotation: Quat,
    pub scale: Vec3,
    pub translation: Vec3,

    updated: bool,
    orig_rotation: Quat,
}
impl Model {
    pub fn new(exam_rotation: Quat) -> Self {
        Self {
            rotation: exam_rotation,
            scale: Vec3::ONE,
            translation: Vec3::ZERO,
            orig_rotation: exam_rotation,
            updated: true,
        }
    }

    pub fn get_model_matrix(&self) -> ModelUniform {
        let model_matrix = self.build_model_matrix();
        ModelUniform {
            model: model_matrix,
            inv_model: model_matrix.inverse(),
        }
    }

    fn build_model_matrix(&self) -> Mat4 {
        let model =
            Mat4::from_scale_rotation_translation(self.scale,
                                                  self.rotation,
                                                  self.translation);

        let rotation = glam::Mat4::from_quat(self.rotation);

        model * rotation
    }

    pub fn reset(&mut self) {
        self.rotation = self.orig_rotation;
        self.scale = Vec3::ONE;
        self.translation = Vec3::ZERO;
        self.updated = true;
    }

    pub fn rotate(&mut self, delta_rotation: Quat) {
        self.rotation *= delta_rotation;
        self.updated = true;
    }

    pub fn scale(&mut self, delta_scale: Vec3) {
        self.scale += delta_scale;
        self.updated = true;
    }

    pub fn translate(&mut self, delta_translation: Vec3) {
        self.translation += delta_translation;
        self.updated = true;
    }

    pub fn set_rotation(&mut self, rotation: Quat) {
        self.rotation = rotation;
        self.updated = true;
    }

    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
        self.updated = true;
    }

    pub fn set_translation(&mut self, translation: Vec3) {
        self.translation = translation;
        self.updated = true;
    }

}

pub struct ModelBinding {
    pub buffer: wgpu::Buffer,
    binding_index: u32
}

impl ModelBinding {
    pub fn bind_group_layout_entry(&self) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: self.binding_index,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: Some(ModelUniform::SIZE),
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
            label: Some("Model Buffer"),
            contents: bytemuck::bytes_of(&ModelUniform::default()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        Self { buffer, binding_index }
    }

    pub fn update(&mut self, queue: &wgpu::Queue, model: &mut Model) {
        if model.updated {
            queue.write_buffer(
                &self.buffer,
                0,
                bytemuck::bytes_of(&model.get_model_matrix()),
            );
            model.updated = false;
        }
    }
}

