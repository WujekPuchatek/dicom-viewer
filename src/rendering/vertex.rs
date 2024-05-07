use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    pub pos: [f32; 4],
    pub tex_coord: [f32; 3],
}

pub fn vertex(pos: [i8; 3], tc: [i8; 3]) -> Vertex {
    Vertex {
        pos: [pos[0] as f32, pos[1] as f32, pos[2] as f32, 1.0],
        tex_coord: [tc[0] as f32, tc[1] as f32, tc[2] as f32],
    }
}

pub fn vertex_2d(pos: [f32; 2], tex_coord: [f32; 3]) -> Vertex {
    Vertex {
        pos: [pos[0], pos[1], 0.0, 1.0],
        tex_coord
    }
}