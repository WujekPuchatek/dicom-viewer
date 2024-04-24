pub trait Renderer {
    fn render(&mut self,
              view: &wgpu::TextureView,
              device: &wgpu::Device,
              queue: &wgpu::Queue,
              command_encoder: &mut wgpu::CommandEncoder);
}