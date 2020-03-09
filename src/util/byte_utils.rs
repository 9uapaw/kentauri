pub fn byte_array_to_u32(byte: &[u8; 4]) -> u32 {
    ((byte[0] as u32) << 0) + ((byte[1] as u32) << 8) + ((byte[2] as u32) << 16)
}
