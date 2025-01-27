use bytemuck::{Pod, Zeroable};
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Sphere {
    pub center: [f32; 3],
    pub radius: f32,

    pub albedo: [f32; 3], // 0
    pub material_id: u32, // 0 = lambertian, 1 = metal, 2 = dielectric

    pub fuzz: f32,             // 1
    pub refraction_index: f32, // 2
    pub _pad1: f32,
    pub _pad2: f32,
}
