use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Ray {
    pub origin: [f32; 3],
    pub bounces_left: u32,
    pub direction: [f32; 3],
    pub _pad2: f32, // pad out to 16 bytes,
    pub color: [f32; 3],
    pub _pad3: f32, // pad out to 16 bytes
}
