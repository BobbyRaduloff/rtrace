use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Ray {
    pub origin: [f32; 3],
    pub direction: [f32; 3],
    pub color: [f32; 3],
    pub bounces: u32,
}
