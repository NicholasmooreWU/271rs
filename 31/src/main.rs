fn main() {
    dbg!(hamming::weight_u8(0x33_u8));
    dbg!(hamming::weight_u8(0x00_u8));
    dbg!(hamming::weight_u8(0xFF_u8)); 
    dbg!(hamming::weight_u64(0x0F0F_0F0F_0F0F_0F0F_u64));
    dbg!(hamming::weight_u64(u64::MAX));
    dbg!(hamming::weight_bytes(vec![0x33_u8, 0xFF_u8]));
    dbg!(hamming::weight_words(vec![0xFFFF_FFFF_FFFF_FFFF_u64, 0_u64]));
    dbg!(hamming::distance_u8(0b1010_1010_u8, 0b0101_0101_u8));
    dbg!(hamming::distance_u8(0x33_u8, 0x00_u8));
    dbg!(hamming::distance_u64(
        0x0F0F_0F0F_0F0F_0F0F_u64,
        0xFFFF_FFFF_FFFF_FFFF_u64
    )); 
    dbg!(hamming::distance_bytes(vec![0x33_u8, 0x00_u8], vec![0x00_u8, 0x33_u8]));
    dbg!(hamming::distance_words(vec![0u64, 0xFFu64], vec![0u64, 0u64]));
}

