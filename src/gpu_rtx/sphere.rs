use bytemuck::{Pod, Zeroable};
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Sphere {
    pub radius: f32,
    pub _pad2: [f32; 3], // offset 20..32 => 32 bytes total
    pub center: [f32; 3],
    pub _pad1: f32, // pad out to 16 bytes
}
