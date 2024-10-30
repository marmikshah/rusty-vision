pub type Index4D = (usize, usize, usize, usize);
pub type Index3D = (usize, usize, usize);
pub type Index2D = (usize, usize);

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum dtype {
    uint8(u8),
    uint16(u16),
    uint64(u64),
    // fp16(f16), (Has some issue in rust)
    fp32(f32),
}
