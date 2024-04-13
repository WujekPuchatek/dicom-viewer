use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3, Vec4};
use wgpu::util::DeviceExt;
use crate::utils::non_zero_sized::NonZeroSized;

//https://learnopengl.com/Getting-started/Coordinate-Systems
//https://www.3dgep.com/understanding-the-view-matrix/#The_View_Matrix'

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CameraUniform {
    pub view_position: Vec4,
    pub proj_view: Mat4,
    pub inv_proj: Mat4,
}
unsafe impl Pod for CameraUniform {}
unsafe impl Zeroable for CameraUniform {}

impl Default for CameraUniform {
    fn default() -> Self {
        Self {
            view_position: Vec4::ZERO,
            proj_view: Mat4::IDENTITY,
            inv_proj: Mat4::IDENTITY,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub zoom: f32,
    pub target: Vec3,
    pub eye: Vec3,
    pub pitch: f32,
    pub yaw: f32,
    pub up: Vec3,
    pub aspect: f32,

    updated: bool,
}
impl Camera {
    const ZFAR: f32 = 10.0;
    const ZNEAR: f32 = 1.0;
    const FOVY: f32 = std::f32::consts::FRAC_PI_4 / 2.0;
    const UP: Vec3 = Vec3::Z;

    const START_EYE: Vec3 = Vec3::new(0.0, -5.0, 0.0);

    pub fn new(zoom: f32, pitch: f32, yaw: f32, target: Vec3, aspect: f32) -> Self {
        let mut camera = Self {
            zoom,
            pitch,
            yaw,
            eye: Self::START_EYE,
            target,
            up: Self::UP,
            aspect,

            updated: false,
        };
        camera.fix_eye();
        camera
    }

    pub fn build_projection_view_matrix(&self) -> Mat4 {
        let view = Mat4::look_at_rh(self.eye, self.target, self.up);
        let proj = Mat4::perspective_rh(Self::FOVY, self.aspect, Self::ZNEAR, Self::ZFAR);
        proj * view
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.clamp(0.3, Self::ZFAR / 2.);
        self.fix_eye();
        self.updated = true;
    }

    pub fn add_zoom(&mut self, delta: f32) {
        self.set_zoom(self.zoom + delta);
    }

    pub fn set_pitch(&mut self, pitch: f32) {
        self.pitch = pitch.clamp(
            -std::f32::consts::PI / 2.0 + f32::EPSILON,
            std::f32::consts::PI / 2.0 - f32::EPSILON,
        );
        self.fix_eye();
        self.updated = true;
    }

    pub fn add_pitch(&mut self, delta: f32) {
        self.set_pitch(self.pitch + delta);
    }

    pub fn set_yaw(&mut self, yaw: f32) {
        self.yaw = yaw;
        self.fix_eye();
        self.updated = true;
    }

    pub fn add_yaw(&mut self, delta: f32) {
        self.set_yaw(self.yaw + delta);
    }

    fn fix_eye(&mut self) {
        let pitch_cos = self.pitch.cos();
        self.eye = self.target
            - self.zoom
            * Vec3::new(
            self.yaw.sin() * pitch_cos,
            self.pitch.sin(),
            self.yaw.cos() * pitch_cos,
        );
    }

    pub fn set_aspect(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
        self.updated = true;
    }

    pub fn get_proj_view_matrix(&self) -> CameraUniform {
        let proj_view = self.build_projection_view_matrix();
        CameraUniform {
            view_position: Vec4::new(self.eye.x, self.eye.y, self.eye.z, 1.0),
            proj_view,
            inv_proj: proj_view.inverse(),
        }
    }
}

pub struct CameraBinding {
    pub buffer: wgpu::Buffer,

    binding_index: u32
}

impl CameraBinding {
    fn bind_group_layout_entry(binding_index: u32) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: binding_index,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: Some(CameraUniform::SIZE),
            },
            count: None,
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

