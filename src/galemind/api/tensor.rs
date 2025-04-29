
pub enum Data {
    BOOL(bool),
    UINT8(u8),
    UINT16(u16),
    UINT32(u32),
    UINT64(u64),
    INT8(i8),
    INT16(i16),
    INT32(i32),
    INT64(i64),
    FP16(f32),
    FP32(f32),
    FP64(f64),
    BYTES(Vec<u8>)
}

pub type DataShape = Vec<usize>;