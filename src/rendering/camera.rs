use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3, Vec4};
use wgpu::util::DeviceExt;
use crate::utils::non_zero_sized::NonZeroSized;

//https://learnopengl.com/Getting-started/Coordinate-Systems
//https://www.3dgep.com/understanding-the-view-matrix/#The_View_Matrix'

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CameraUniform {
    pub eye_position: Vec4,
    pub proj_view: Mat4,
    pub inv_proj_view: Mat4,
}
unsafe impl Pod for CameraUniform {}
unsafe impl Zeroable for CameraUniform {}

impl Default for CameraUniform {
    fn default() -> Self {
        Self {
            eye_position: Vec4::ZERO,
            proj_view: Mat4::IDENTITY,
            inv_proj_view: Mat4::IDENTITY,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub near: f32,
    pub far: f32,
    pub fovy : f32,
    pub aspect: f32,

    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,

    updated: bool,
}
impl Camera {
    const ZFAR: f32 = 100.0;
    const ZNEAR: f32 = 1.0;
    const FOVY: f32 = std::f32::consts::FRAC_PI_4;
    const UP: Vec3 = Vec3::Z;
    const TARGET: Vec3 = Vec3::ZERO;
    const EYE: Vec3 = Vec3::new(0.0, -4.0, 0.0);

    pub fn new(aspect: f32) -> Self {
        Self {
            near: Self::ZNEAR,
            far: Self::ZFAR,
            fovy: Self::FOVY,
            aspect,

            eye: Self::EYE,
            target : Self::TARGET,
            up: Self::UP,

            updated: false,
        }
    }
    pub fn get_proj_view_matrix(&self) -> CameraUniform {
        let proj_view = self.generate_projection_matrix() * self.generate_view_matrix();

        CameraUniform {
            eye_position: Vec4::new(self.eye.x, self.eye.y, self.eye.z, 1.0),
            proj_view,
            inv_proj_view: proj_view.inverse(),
        }
    }
    pub fn zoom_delta(&mut self, zoom_delta: f32) {
        self.fovy += zoom_delta;
        self.fovy = self.fovy.clamp(0.25, 5.0);

        self.updated = true;
    }

    pub fn set_aspect(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;

        self.updated = true;
    }

    pub fn move_forward(&mut self, delta: f32) {
        let forward = (self.target - self.eye).normalize();
        self.eye += forward * delta;
        self.target += forward * delta;
        self.updated = true;
    }

    pub fn move_right(&mut self, delta: f32) {
        let start_distance = (self.target - self.eye).length();

        let mut forward = (self.target - self.eye).normalize();
        let right = forward.cross(Self::UP).normalize();
        self.eye += right * delta;

        forward = (self.target - self.eye).normalize();
        self.target = self.eye + forward * start_distance;

        self.updated = true;
    }

    pub fn move_up(&mut self, delta: f32) {
        self.eye += Self::UP * delta;
        self.updated = true;
    }

    pub fn up(&self) -> Vec3 {
        Self::UP
    }

    pub fn eye(&self) -> Vec3 {
        Self::EYE
    }

    fn generate_projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fovy,
                             self.aspect,
                             self.near,
                             self.far)
    }

    fn generate_view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.eye,
                         self.target,
                         self.up)
    }
}

pub struct CameraBinding {
    pub buffer: wgpu::Buffer,

    binding_index: u32
}

impl CameraBinding {
    pub fn bind_group_layout_entry(&self) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: self.binding_index,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: Some(CameraUniform::SIZE),
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
            label: Some("Camera Buffer"),
            contents: bytemuck::bytes_of(&CameraUniform::default()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        Self { buffer, binding_index }
    }

    pub fn update(&mut self, queue: &wgpu::Queue, camera: &mut Camera) {
        if camera.updated {
            queue.write_buffer(
                &self.buffer,
                0,
                bytemuck::bytes_of(&camera.get_proj_view_matrix()),
            );
            camera.updated = false;
        }
    }
}

