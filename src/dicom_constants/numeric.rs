pub const HEADER_LENGTH: usize = 4;
pub const HEADER_START: usize = 128;
pub const HEADER_END : usize = HEADER_START + HEADER_LENGTH;
pub const HEADER: &[u8; HEADER_LENGTH] = b"DICM";
