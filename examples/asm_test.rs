use endianness::Endianness;

#[no_mangle]
pub fn test_u16_conversion(val: u16) -> u16 {
    val.host_to_big_endian()
}

#[no_mangle]
pub fn test_u32_conversion(val: u32) -> u32 {
    val.host_to_little_endian()
}

#[no_mangle]
pub fn test_u64_conversion(val: u64) -> u64 {
    val.big_endian_to_host()
}

fn main() {
    println!("assembly test functions compiled successfully");
}
