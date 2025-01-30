#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SceneUniforms {
    pub pass_idx: u32,
    pub bounce_idx: u32,
}
