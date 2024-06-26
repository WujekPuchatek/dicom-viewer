use wgpu::{BindingResource, SamplerBindingType};

#[derive(Debug, Clone, Copy)]
pub struct Sampler {
    address_mode: wgpu::AddressMode,

    mag_filter: wgpu::FilterMode,
    min_filter: wgpu::FilterMode,
    mipmap_filter: wgpu::FilterMode,
}

impl Sampler {
    pub fn new() -> Self {
        Self {
            address_mode: wgpu::AddressMode::ClampToEdge,

            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
        }
    }
}

pub struct SamplerBinding {
    sampler: wgpu::Sampler,
    binding_index: u32
}

impl SamplerBinding {
    pub fn bind_group_layout_entry(&self) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: self.binding_index,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler (SamplerBindingType::Filtering),
            count: None,
        }
    }

    pub fn bind_group_entry(&self) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding: self.binding_index,
            resource: BindingResource::Sampler(&self.sampler)
        }
    }

    pub fn new(device: &wgpu::Device, binding_index: u32, sampler: Sampler) -> Self {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Sampler"),
            address_mode_u: sampler.address_mode,
            address_mode_v: sampler.address_mode,
            address_mode_w: sampler.address_mode,
            mag_filter: sampler.mag_filter,
            min_filter: sampler.min_filter,
            mipmap_filter: sampler.mipmap_filter,
            ..Default::default()
        });

        Self { sampler, binding_index }
    }
}

