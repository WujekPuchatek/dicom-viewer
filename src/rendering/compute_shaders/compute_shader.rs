use wgpu::CommandEncoder;

pub trait ComputeShader {
    fn step(&self, device: &wgpu::Device, queue: &wgpu::Queue, command_encoder: &mut CommandEncoder);

    fn compute_work_group_count(
        dimensions: (u32, u32, u32),
        work_group_size: (u32, u32, u32),
    ) -> (u32, u32, u32) {
        (
            (dimensions.0 + work_group_size.0 - 1) / work_group_size.0,
            (dimensions.1 + work_group_size.1 - 1) / work_group_size.1,
            (dimensions.2 + work_group_size.2 - 1) / work_group_size.2,
        )
    }
}